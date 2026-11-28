// Input posted straight to a window's message queue, for captures that run in the background:
// the window takes clicks, keys, text and the wheel while it sits off-screen, and the real
// pointer, keyboard and foreground window stay with whoever is using the computer.
//
// gpui reads modifiers with GetKeyState and text with GetKeyboardState, so modifier presses join
// this thread's input state to the window's thread (AttachThreadInput) and set it there.
using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;

public static class SimPost {
    [DllImport("user32.dll")] static extern bool PostMessageW(IntPtr h, uint msg, IntPtr w, IntPtr l);
    [DllImport("user32.dll")] static extern IntPtr SendMessageTimeoutW(IntPtr h, uint msg, IntPtr w, IntPtr l, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern IntPtr SendMessageW(IntPtr h, uint msg, IntPtr w, string l);
    [DllImport("user32.dll")] static extern bool GetKeyboardState(byte[] state);
    [DllImport("user32.dll")] static extern bool SetKeyboardState(byte[] state);
    [DllImport("user32.dll")] static extern uint MapVirtualKeyW(uint code, uint type);
    [DllImport("user32.dll")] static extern short VkKeyScanW(char c);
    [DllImport("user32.dll")] static extern bool AttachThreadInput(uint a, uint b, bool attach);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("kernel32.dll")] static extern uint GetCurrentThreadId();
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr parent, EnumProc f, IntPtr l);
    [DllImport("user32.dll")] static extern IntPtr GetParent(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassNameW(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern IntPtr GetDlgItem(IntPtr dlg, int id);
    [DllImport("user32.dll")] public static extern bool IsZoomed(IntPtr h);
    [DllImport("user32.dll")] static extern uint WaitForInputIdle(IntPtr process, uint ms);
    [DllImport("kernel32.dll")] static extern IntPtr OpenProcess(uint access, bool inherit, uint pid);
    [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
    delegate bool EnumProc(IntPtr h, IntPtr l);
    const uint SYNCHRONIZE = 0x00100000, PROCESS_QUERY_INFORMATION = 0x0400, IdleWaitMs = 500;

    const uint WM_NULL = 0x0000, WM_ACTIVATE = 0x0006, WM_SETTEXT = 0x000C, WM_COMMAND = 0x0111;
    const uint WM_KEYDOWN = 0x0100, WM_KEYUP = 0x0101, WM_CHAR = 0x0102, WM_SYSKEYDOWN = 0x0104, WM_SYSKEYUP = 0x0105;
    const uint WM_MOUSEMOVE = 0x0200, WM_LBUTTONDOWN = 0x0201, WM_LBUTTONUP = 0x0202;
    const uint WM_RBUTTONDOWN = 0x0204, WM_RBUTTONUP = 0x0205, WM_MOUSEWHEEL = 0x020A;
    const int MK_LBUTTON = 0x0001, MK_RBUTTON = 0x0002;
    public const int Shift = 1, Ctrl = 2, Alt = 4;
    const uint VK_RETURN = 0x0D;
    // Pressed in this order and released in reverse, like a person holding a chord.
    static readonly int[] Chord = { Ctrl, Shift, Alt };
    const int ChordHoldMs = 120;

    static IntPtr Point(int x, int y) { return new IntPtr(unchecked((int)(((uint)(y & 0xFFFF) << 16) | (uint)(x & 0xFFFF)))); }

    /// Returns once the window's thread has handled what was posted to it. Sent messages alone
    /// do not prove it: the thread answers them at any message read, including reads it makes
    /// while still busy with an earlier key. So a sent message wakes it, then this waits for
    /// it to go idle, which it does only with an empty queue.
    public static void Flush(IntPtr h) {
        IntPtr r;
        SendMessageTimeoutW(h, WM_NULL, IntPtr.Zero, IntPtr.Zero, 0x0002, 5000, out r);
        uint pid; GetWindowThreadProcessId(h, out pid);
        IntPtr process = OpenProcess(SYNCHRONIZE | PROCESS_QUERY_INFORMATION, false, pid);
        if (process != IntPtr.Zero) { WaitForInputIdle(process, IdleWaitMs); CloseHandle(process); }
        SendMessageTimeoutW(h, WM_NULL, IntPtr.Zero, IntPtr.Zero, 0x0002, 5000, out r);
    }

    /// Tells the window it is the active one (caret, focus rings) without taking the foreground.
    public static void Activate(IntPtr h) { PostMessageW(h, WM_ACTIVATE, new IntPtr(1), IntPtr.Zero); Flush(h); }

    public static void Move(IntPtr h, int x, int y, bool leftHeld) {
        PostMessageW(h, WM_MOUSEMOVE, new IntPtr(leftHeld ? MK_LBUTTON : 0), Point(x, y)); Flush(h);
    }

    public static void Button(IntPtr h, int x, int y, bool right, bool down) {
        uint msg = right ? (down ? WM_RBUTTONDOWN : WM_RBUTTONUP) : (down ? WM_LBUTTONDOWN : WM_LBUTTONUP);
        int held = down ? (right ? MK_RBUTTON : MK_LBUTTON) : 0;
        PostMessageW(h, msg, new IntPtr(held), Point(x, y)); Flush(h);
    }

    /// A wheel turn at screen point (sx, sy): 120 units a notch, negative scrolls down.
    public static void Wheel(IntPtr h, int sx, int sy, int delta) {
        PostMessageW(h, WM_MOUSEWHEEL, new IntPtr(unchecked((int)((uint)(delta & 0xFFFF) << 16))), Point(sx, sy)); Flush(h);
    }

    /// Drags with the left button through client points, `stepMs` apart.
    public static void Drag(IntPtr h, int[] xs, int[] ys, int stepMs) {
        int last = xs.Length - 1;
        Move(h, xs[0], ys[0], false);
        Button(h, xs[0], ys[0], false, true);
        for (int i = 0; i <= last; i++) { Move(h, xs[i], ys[i], true); if (stepMs > 0) Thread.Sleep(stepMs); }
        Button(h, xs[last], ys[last], false, false);
    }

    static IntPtr KeyBits(uint vk, bool up, bool extended, bool alt) {
        uint l = 1u | (MapVirtualKeyW(vk, 0) << 16);
        if (extended) l |= 1u << 24;
        if (alt) l |= 1u << 29;
        if (up) l |= (1u << 30) | (1u << 31);
        return new IntPtr(unchecked((int)l));
    }

    static void PostKey(IntPtr h, uint vk, bool up, bool extended, int mods) {
        bool alt = (mods & Alt) != 0, sys = alt && (mods & Ctrl) == 0;
        uint msg = up ? (sys ? WM_SYSKEYUP : WM_KEYUP) : (sys ? WM_SYSKEYDOWN : WM_KEYDOWN);
        PostMessageW(h, msg, new IntPtr((int)vk), KeyBits(vk, up, extended, alt));
    }

    static void SetHeld(byte[] ks, int mods) {
        byte s = (byte)((mods & Shift) != 0 ? 0x80 : 0), c = (byte)((mods & Ctrl) != 0 ? 0x80 : 0), a = (byte)((mods & Alt) != 0 ? 0x80 : 0);
        ks[0x10] = s; ks[0xA0] = s; ks[0x11] = c; ks[0xA2] = c; ks[0x12] = a; ks[0xA4] = a;
        SetKeyboardState(ks);
    }

    static uint ModifierKey(int bit) { return bit == Ctrl ? 0x11u : bit == Shift ? 0x10u : 0x12u; }

    /// Presses and releases modifier keys until `want` is held, each handled before the next.
    static int Hold(IntPtr h, byte[] ks, int held, int want) {
        for (int i = Chord.Length - 1; i >= 0; i--) {
            int bit = Chord[i];
            if ((held & bit) == 0 || (want & bit) != 0) continue;
            int before = held; held &= ~bit;
            SetHeld(ks, held); PostKey(h, ModifierKey(bit), true, false, before); Flush(h);
        }
        foreach (int bit in Chord) {
            if ((held & bit) != 0 || (want & bit) == 0) continue;
            held |= bit;
            SetHeld(ks, held); PostKey(h, ModifierKey(bit), false, false, held); Flush(h);
        }
        return held;
    }

    /// Runs `body` with this thread's input state joined to the window's thread, so the key
    /// state set here is the one the window reads; nothing stays held afterwards.
    static void Joined(IntPtr h, Action<byte[]> body) {
        uint pid; uint target = GetWindowThreadProcessId(h, out pid), me = GetCurrentThreadId();
        bool joined = AttachThreadInput(me, target, true);
        byte[] ks = new byte[256];
        GetKeyboardState(ks);
        try { body(ks); }
        finally { SetHeld(ks, 0); if (joined) AttachThreadInput(me, target, false); }
    }

    /// One key, `count` times, with modifiers held (Shift | Ctrl | Alt). The flush returns once
    /// the window has read the key, which may be before it has finished with it, so a chord
    /// stays held a moment longer.
    public static void Key(IntPtr h, uint vk, int mods, bool extended, int count) {
        Joined(h, ks => {
            int held = Hold(h, ks, 0, mods);
            // One press at a time, each handled before the next, so none outlives the chord.
            for (int i = 0; i < count; i++) { PostKey(h, vk, false, extended, mods); PostKey(h, vk, true, extended, mods); Flush(h); }
            if (mods != 0) Thread.Sleep(ChordHoldMs);
            Hold(h, ks, held, 0);
        });
    }

    /// Types text. One plain character (a digit, a lower-case letter, a space) is a key press,
    /// so a key binding sees it as typed. Anything longer arrives as characters (WM_CHAR), in
    /// order: Windows makes a key press's character only after the window has handled the key,
    /// too late for a Shift set here, and behind characters posted meanwhile. A line break is
    /// an Enter press.
    public static void Text(IntPtr h, string text) {
        if (text.Length == 1 && text[0] != '\n') {
            short scan = VkKeyScanW(text[0]);
            if (scan != -1 && ((scan >> 8) & 0xFF) == 0) { Key(h, (uint)(scan & 0xFF), 0, false, 1); return; }
        }
        foreach (char c in text) {
            if (c == '\r') continue;
            if (c == '\n') { Flush(h); Key(h, VK_RETURN, 0, false, 1); continue; }
            PostMessageW(h, WM_CHAR, new IntPtr(c), new IntPtr(1));
        }
        Flush(h);
    }

    /// The layout's key for a character: virtual key in the low byte, Shift/Ctrl/Alt bits above.
    public static short ScanOf(char c) { return VkKeyScanW(c); }

    static string ClassOf(IntPtr h) { var s = new StringBuilder(256); GetClassNameW(h, s, 256); return s.ToString(); }

    /// Fills the file name box of a common open/save dialog and presses its default button, by
    /// message, so the dialog needs neither the foreground nor the keyboard.
    public static bool FileDialog(IntPtr dlg, string path) {
        IntPtr edit = IntPtr.Zero;
        EnumChildWindows(dlg, (c, l) => {
            if (ClassOf(c) == "Edit" && ClassOf(GetParent(c)) == "ComboBox") { edit = c; return false; }
            return true;
        }, IntPtr.Zero);
        if (edit == IntPtr.Zero) return false;
        SendMessageW(edit, WM_SETTEXT, IntPtr.Zero, path);
        PostMessageW(dlg, WM_COMMAND, new IntPtr(1), GetDlgItem(dlg, 1));   // IDOK, BN_CLICKED
        return true;
    }
}
