//! The Lucide icons [`LucideIcon`](crate::ui::icons::LucideIcon) maps to, embedded one by one
//! instead of the kit's full ~1 MiB catalog. Add the kit variant here when the catalog grows;
//! `ui::assets::tests` fails for any catalog path this set (or the kit defaults) cannot serve.

gpui_kit::assets::icon_assets!(
    pub CatalogIcons,
    [
        Activity, Archive, ArrowDown, ArrowLeft, ArrowRight, ArrowUp, BookOpen, Bot, Boxes, Brain,
        Calendar, Check, ChevronDown, ChevronLeft, ChevronRight, ChevronUp, ChevronsUpDown,
        CircleQuestionMark, Clock, Compass, Copy, Cpu, Dices, Ellipsis, ExternalLink, Eye, FileText,
        Folder, Funnel, GitBranch, Globe, Hash, Heart, Image, Info, Keyboard, Library, Link2, Lock, Maximize2,
        MessageSquare, Minimize2, Minus, Moon, Palette, PanelLeft, PanelRight, Pin, Plus, RefreshCw,
        Repeat, Search, Send, Settings, Share2, SlidersHorizontal, Sparkles, SquarePen, Sun, SunMoon, Tag,
        Terminal, Trash, User, UserCheck, Users, Volume2, Wifi, X, Zap,
        // sb A2
        Command, CornerDownLeft, FileUp, MessageSquarePlus, PanelLeftClose, PanelLeftOpen, PinOff, Star,
        StickyNote, UserPlus,
        // sb B2
        Feather, Pilcrow, VenetianMask,
    ]
);
