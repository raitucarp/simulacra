// A hand-written stand-in for one icon set's module: two icons and one block that is not.
import { GenIcon } from '../lib/index.mjs';
export function FiZoomOut (props) {
  return GenIcon({"tag":"svg","attr":{"viewBox":"0 0 24 24","fill":"none","stroke":"currentColor","strokeWidth":"2","strokeLinecap":"round"},"child":[{"tag":"circle","attr":{"cx":"11","cy":"11","r":"8"},"child":[]},{"tag":"line","attr":{"x1":"8","y1":"11","x2":"14","y2":"11"},"child":[]}]})(props);
};
export function FiZap (props) {
  return GenIcon({"tag":"svg","attr":{"viewBox":"0 0 24 24","fill":"currentColor"},"child":[{"tag":"script","attr":{},"child":[]},{"tag":"polygon","attr":{"points":"13 2 3 14 12 14 11 22 21 10 12 10 13 2","onload":"x"},"child":[]}]})(props);
};
export function FiBroken (props) {
  return GenIcon({"tag":"svg","attr":{},"child":[]})(props);
};
