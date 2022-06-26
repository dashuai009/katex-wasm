pub trait VirtualNode {
    fn to_node(&self) -> web_sys::Node;
    fn to_markup(&self) -> String;
}

//export interface HtmlDomNode extends VirtualNode {
//     classes: string[];
//     height: number;
//     depth: number;
//     maxFontSize: number;
//     style: CssStyle;
//
//     hasClass(className: string): boolean;
// }
pub trait HasClassNode {
    fn has_class(&self, class_name: &String) -> bool;
}
pub trait HtmlDomNode: VirtualNode + HasClassNode {}
