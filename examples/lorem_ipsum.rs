//! Pager example.
//!
//! ```shell
//! cargo run --example lorem_ipsum
//! ```

use lessify::Pager;

fn main() {
    let text = text();

    Pager::page_or_print(text);
}

fn text() -> &'static str {
    "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut ex suscipit, elementum tortor et, pretium odio. Suspendisse quis lacus vel nulla mollis malesuada. Sed orci purus, auctor ut tempor vitae, convallis ut lorem. Donec sem augue, efficitur condimentum mauris non, sagittis porttitor lacus. Proin maximus suscipit pellentesque. Vestibulum ante eros, ultrices ac varius a, porttitor nec libero. Proin tempus dui vel leo rutrum eleifend.

Nullam est libero, posuere vitae tellus ut, volutpat aliquam erat. Fusce vitae urna nibh. Proin luctus, augue non aliquam elementum, purus magna consequat ligula, id lobortis magna leo nec nunc. Donec at turpis dapibus, malesuada massa vitae, fringilla ligula. Aenean ut ullamcorper nulla. Phasellus ex est, maximus eget lectus id, aliquet blandit dolor. Duis at mauris in eros ornare placerat quis posuere massa. Suspendisse potenti. Vestibulum malesuada ligula dolor, non placerat felis venenatis vel. Nam convallis purus et justo pellentesque, nec ultricies odio sollicitudin.

Quisque cursus velit sit amet euismod sodales. Donec sit amet vehicula nulla. Quisque at diam at urna eleifend auctor et non mi. Mauris sed sapien consequat, hendrerit nibh non, cursus lacus. Sed suscipit augue turpis, ut ullamcorper mauris volutpat eu. Pellentesque a lectus mattis, vehicula dui quis, ornare augue. Curabitur a placerat elit, at cursus diam. Mauris dignissim tortor sit amet vulputate imperdiet. Sed nec lacinia odio, non vehicula augue. Mauris dapibus erat id vestibulum feugiat. Sed rhoncus et velit sit amet semper. Morbi varius pharetra ex et vulputate. Aenean scelerisque condimentum ornare. Suspendisse laoreet commodo vulputate. Curabitur eu scelerisque leo.

Nulla facilisi. Sed ullamcorper tellus eget erat facilisis, non accumsan dolor ornare. Nullam ornare tempor nulla, at malesuada nibh malesuada sit amet. Donec elit ipsum, finibus in auctor id, tempor ut ipsum. Curabitur at pretium sem, quis interdum massa. Cras a aliquam tellus, vel elementum nisi. Praesent finibus quis nibh at tincidunt. Nam euismod diam tempus, tempor dui hendrerit, feugiat diam. Nam pulvinar in enim et lobortis. Pellentesque lobortis eget risus ut consectetur.

Vestibulum vehicula nisl eu nibh aliquam sollicitudin. Proin suscipit nunc egestas risus congue sollicitudin. Integer venenatis vitae nunc sit amet sollicitudin. Etiam dictum ligula et nulla feugiat auctor. Aliquam erat volutpat. Ut vel tincidunt tortor, ac efficitur eros. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Sed lacinia nisl vitae vestibulum suscipit. Integer volutpat rhoncus sapien, ut tempor urna. Nulla egestas, purus vitae porttitor eleifend, orci risus porttitor arcu, id laoreet erat quam et dui. Vestibulum sed massa neque. Curabitur vel libero aliquam, ultrices est tempus, porta dui.

Vivamus et maximus risus, eu posuere purus. Fusce cursus mauris eu erat aliquet volutpat. Aliquam in ultrices sapien, a mattis orci. Vestibulum sollicitudin, elit sit amet consectetur laoreet, lectus massa mollis tellus, interdum rutrum enim nisi at justo. Etiam nulla est, tristique vitae diam congue, placerat porta sapien. Fusce ut placerat tortor. Vivamus vel consequat erat. Aliquam gravida ultricies orci, cursus dictum mi porta vel. Donec malesuada finibus ante, non efficitur mi tempor in. Sed at luctus est, eu congue massa. Nulla ligula justo, dapibus at convallis vitae, consectetur lacinia nunc. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur blandit consequat nulla eget egestas. Praesent laoreet sapien sit amet pretium dapibus. Quisque laoreet consectetur justo eget scelerisque.

Quisque venenatis, ipsum dapibus semper porttitor, elit eros facilisis elit, sit amet congue tortor sem sed ipsum. Vestibulum tincidunt vitae nisi ac fringilla. Fusce gravida odio nec neque cursus porta. Nam ac quam est. Nulla porta dapibus arcu ut venenatis. Fusce molestie posuere porta. Proin placerat ipsum vel efficitur ullamcorper. Sed suscipit ullamcorper ante sed commodo. Nullam fermentum molestie sem non rutrum. Praesent vitae pretium nulla, vel egestas velit. Aliquam erat volutpat. Ut in leo vehicula, ullamcorper mauris ac, euismod ipsum. Suspendisse id pharetra nisi. Maecenas vitae pulvinar orci, sed vestibulum tortor. Vestibulum et mollis ex, varius consequat massa.

Fusce vestibulum nunc in viverra consectetur. Sed quis leo metus. Fusce imperdiet diam id quam volutpat, ac dapibus tellus placerat. Sed rhoncus feugiat luctus. Nulla turpis sapien, commodo eu ornare ac, sodales non erat. Maecenas a nunc eu nibh tempus accumsan. Vestibulum ante leo, volutpat vulputate efficitur at, tristique vitae massa. Nullam mattis augue sed ultricies efficitur.

Cras et magna tellus. Nullam est sapien, mattis non magna nec, hendrerit posuere ante. Nam sed quam sapien. Curabitur pharetra facilisis viverra. Mauris quis neque ultrices, eleifend erat ac, blandit nisl. Cras ut enim aliquet mi sagittis sagittis sed non dolor. Vivamus iaculis pharetra libero. Proin arcu dui, volutpat porta magna vel, tincidunt gravida sem. Donec ultrices ex in sollicitudin lobortis. Ut euismod placerat urna a iaculis. Proin tincidunt elementum mi in scelerisque.

Donec dictum, nulla quis vehicula faucibus, orci nunc cursus erat, et volutpat tortor sem sed tortor. Sed quam quam, commodo sit amet ligula non, eleifend auctor ante. Donec blandit molestie accumsan. Nullam posuere laoreet dictum. Aenean efficitur dictum risus, ac maximus ante sollicitudin ut. Aenean consectetur eget est eu hendrerit. Duis sagittis ante enim, egestas malesuada dolor scelerisque scelerisque. Nam laoreet posuere lorem. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aenean est dui, scelerisque sed sapien et, condimentum sollicitudin nibh. In dignissim tellus enim. Curabitur non ligula fringilla, eleifend felis vitae, cursus augue. Etiam vestibulum mauris non ante scelerisque rutrum.
"
}
