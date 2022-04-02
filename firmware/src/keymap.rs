use crate::custom_action::PkbAction;
use crate::custom_action::CustomActionState;
use crate::keyboard::MediaKey;
use keyberon::action::{d, k, l, m, Action, Action::*};
use keyberon::key_code::KeyCode::*;
use serde::{Deserialize, Serialize};

const PLAY_PAUSE: Action<PkbAction> = Custom(PkbAction::MediaKey(MediaKey::PlayPause));
const NEXT: Action<PkbAction> = Custom(PkbAction::MediaKey(MediaKey::NextTrack));
const PREVIOUS: Action<PkbAction> = Custom(PkbAction::MediaKey(MediaKey::PrevTrack));

const START_CMDT: Action<PkbAction> = MultipleActions(&[
    Custom(PkbAction::HoldCmd),
    d(Layer::Tabbing as usize),
    k(Tab),
]);
const END_CMDT: Action<PkbAction> =
    MultipleActions(&[Custom(PkbAction::ReleaseCmd), d(Layer::Default as usize)]);

const START_CTRLT: Action<PkbAction> = MultipleActions(&[
    Custom(PkbAction::HoldCtrl),
    d(Layer::Tabbing as usize),
    k(Tab),
]);
const END_CTRLT: Action<PkbAction> =
    MultipleActions(&[Custom(PkbAction::ReleaseCtrl), d(Layer::Default as usize)]);
const SHFT_TAB: Action<PkbAction> = m(&[LShift, Tab]);

const MENU_OPEN: Action<PkbAction> =
    MultipleActions(&[Custom(PkbAction::MenuOpen), d(Layer::Menu as usize)]);
const MENU_UP: Action<PkbAction> = Custom(PkbAction::MenuUp);
const MENU_DOWN: Action<PkbAction> = Custom(PkbAction::MenuDown);
const MENU_LEFT: Action<PkbAction> = Custom(PkbAction::MenuLeft);
const MENU_RIGHT: Action<PkbAction> = Custom(PkbAction::MenuRight);
const MENU_SELECT: Action<PkbAction> = Custom(PkbAction::MenuSelect);
const MENU_CLOSE: Action<PkbAction> =
    MultipleActions(&[Custom(PkbAction::MenuClose), d(Layer::Default as usize)]);

const OS_TOGGLE: Action<PkbAction> = Custom(PkbAction::ToggleMacMode);

// keycode for symbol layer
const SYMBOLS: Action<PkbAction> = Custom(PkbAction::SymbolsLayer);

macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

macro_rules! ralt {
    ($k:ident) => {
        m(&[RAlt, $k])
    };
}

macro_rules! lalt {
    ($k:ident) => {
        m(&[LAlt, $k])
    };
}

macro_rules! salt {
    ($k:ident) => {
        m(&[LAlt, LShift, $k])
    };
}

// scrap this
macro_rules! symbols {
    ($k:ident) => {
        match CustomActionState::get_mac_mode() {
            true => l(Symbols),
            _ => l(Symbols)
        }
    };
}

const LC: Action<PkbAction> = s!(LBracket);
const RC: Action<PkbAction> = s!(RBracket);

const LS: Action<PkbAction> = k(LBracket);
const RS: Action<PkbAction> = k(RBracket);

const LB: Action<PkbAction> = s!(Kb9);
const RB: Action<PkbAction> = s!(Kb0);

const SC: Action<PkbAction> = k(SColon);
const CO: Action<PkbAction> = s!(SColon);

const DQ: Action<PkbAction> = s!(Quote);
const QU: Action<PkbAction> = k(Quote);

const HASH: Action<PkbAction> = m(&[LAlt, Kb3]);
const TILDA: Action<PkbAction> = s!(Grave);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<PkbAction> = &[
   // &[
   //     &[k(Tab),     k(Q),         k(W),     k(F),       k(P),          k(B),      k(Escape),          k(Insert),  k(J),     k(L),              k(U),        k(Y),      k(Quote),   k(SColon)],
   //     &[k(LCtrl),   k(A),         k(R),     k(S),       k(T),          k(G),      MENU_OPEN,          k(Delete),  k(M),     k(N),              k(E),        k(I),      k(O),       k(Bslash)],
   //     &[k(LShift),  k(Z),         k(X),     k(C),       k(D),          k(V),      k(Mute),            PLAY_PAUSE, k(K),     k(H),              k(Comma),    k(Dot),    k(Slash),   k(RShift)],
   //     &[k(VolUp),   k(VolDown),   k(LAlt),  k(LGui),    l(1),          k(Enter),  k(LShift),          k(RShift),  k(Space), l(2),              k(RCtrl),    k(RAlt),   PREVIOUS,   NEXT],
   // ], 
    &[
        &[k(Escape),k(Q),    k(W),     k(E),    k(R),   k(T),    k(Delete), k(Enter),  k(Y),     k(U),              k(I),        k(O),      k(P),   k(LBracket)],
        &[k(Tab),   k(A),    k(S),     k(D),    k(F),   k(G),    MENU_OPEN,          k(CapsLock),  k(H),     k(J),              k(K),        k(L),      k(SColon),       k(Quote)],
        &[k(LShift),k(Z),    k(X),     k(C),    k(V),   k(B),    k(Application),      k(Insert), k(N),     k(M),              k(Comma),    k(Dot),    k(Slash),   k(RShift)],
        &[k(Down),  k(Up),   k(LCtrl), k(LGui), k(LAlt), k(Space), SYMBOLS,        SYMBOLS, k(BSpace),  k(RAlt),              k(RGui),    k(RCtrl),   k(Left),   k(Right)],
        &[l(1), l(2),   NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp]  // l.event
    ],
    // Symbols
    &[
        &[k(Grave),      ralt!(Kb8),         ralt!(Kb9),     k(Up),       s!(Kb8),            s!(Kb9),              k(Minus),             s!(Kb0),       s!(Slash),     k(Slash),   k(Up),   k(RBracket),   s!(RBracket),   ralt!(RBracket)],
        &[Trans,      s!(Equal),         k(Left),       k(Down),         k(Right),           k(Equal),             k(Insert),               Trans,       ralt!(Minus),   k(Left),   k(Down),    k(Right),   s!(Minus),  ralt!(Kb4)],
        &[Trans,      k(NonUsBslash),         s!(NonUsBslash),     ralt!(NonUsBslash),   ralt!(Kb7),     ralt!(Kb0),      k(Insert),           Trans,  s!(NonUsHash),     k(NonUsHash),     s!(Kb5),   s!(Kb6),      s!(Kb7),       Trans],
        &[k(PgDown),      k(PgUp),        Trans,    Trans,      Trans,           k(Delete),         l(2),           l(2),    k(Insert),     Trans,             Trans,       Trans,      k(Home),     k(End)],
        &[l(1), l(2),   NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp]
    ],
    // Mac Symbols
    &[
        &[k(Grave),      lalt!(Kb8),         lalt!(Kb9),     k(Up),       s!(Kb8),            s!(Kb9),              k(Minus),             s!(Kb0),       s!(Slash),     k(Slash),   k(Up),   k(RBracket),   s!(RBracket),   lalt!(RBracket)],
        &[Trans,      s!(Equal),         k(Left),       k(Down),         k(Right),           k(Equal),             k(Insert),               Trans,       lalt!(Kb7),   k(Left),   k(Down),    k(Right),   s!(Minus),  ralt!(Kb4)],
        &[Trans,      k(NonUsBslash),         s!(NonUsBslash),     lalt!(NonUsBslash),   lalt!(Kb8),     lalt!(Kb9),      k(Insert),           Trans,  s!(NonUsHash),     k(NonUsHash),     s!(Kb5),   s!(Kb6),      s!(Kb7),       Trans],
        &[k(PgDown),      k(PgUp),        Trans,    Trans,      Trans,           k(Delete),         l(1),           l(1),    k(Insert),     Trans,             Trans,       Trans,      k(Home),     k(End)],
        &[l(1), l(2),   NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp]
    ],
    &[  
        &[Trans,      k(Kb1),       k(Kb2),   k(Kb3),     k(Kb4),        k(Kb5),    OS_TOGGLE,       Trans,       k(Kb6),   k(Kb7),            k(Kb8),      k(Kb9),    k(Kb0),     k(Minus)],
        &[k(F1),   k(F2),    k(F3),      k(F4),         k(F5),     k(F6),      Trans,            Trans,        k(F7),      k(F8),    k(F9),             k(F10),      k(F11),    k(F12)],
        &[Trans,      Trans,     Trans,     Trans,       Trans,      Trans,  Trans,                 Trans,      Trans,     Trans,       Trans, Trans,      Trans,       Trans],
        &[Trans,      Trans,        Trans,    Trans,      Trans,         Trans,     Trans,                   Trans,      Trans, Trans,             Trans,       Trans,     Trans,      Trans],
        &[l(1), l(2),   NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp]
    ],
    &[
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      NoOp,               NoOp,       NoOp,     NoOp,              k(Up),       NoOp,      NoOp,       NoOp],
        &[Trans,      k(Home),      k(PgUp),  k(PgDown),  k(End),        NoOp,      NoOp,               NoOp,       NoOp,     k(Left),           k(Down),     k(Right),  NoOp,       NoOp],
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      START_CTRLT,        START_CMDT, NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      Trans,        Trans,    Trans,      Trans,         Trans,     Trans,              Trans,      Trans,    Trans,             Trans,       Trans,     Trans,      Trans],
    ],
    &[
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      NoOp,               NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      NoOp,               NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      END_CTRLT,          END_CMDT,   NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[k(Tab),     SHFT_TAB,     Trans,    Trans,      Trans,         Trans,     Trans,              Trans,      Trans,    Trans,             Trans,       Trans,     k(Left),    k(Right)],
    ],
    &[
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      NoOp,               NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      MENU_CLOSE,         NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,          NoOp,      MENU_SELECT,        NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[MENU_DOWN,  MENU_UP,      Trans,    Trans,      Trans,         Trans,     Trans,              Trans,      Trans,    Trans,             Trans,       Trans,     MENU_LEFT,  MENU_RIGHT],
    ],
    // CS
    &[
        &[k(Tab),     k(F),         k(Kb3),   k(W),       k(E),          k(R),      k(Escape),          NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      k(LShift),    k(A),     k(S),       k(D),          k(G),      MENU_OPEN,          NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[Trans,      k(LCtrl),     k(X),     k(T),       k(Kb5),        k(B),      k(Mute),            NoOp,       NoOp,     NoOp,              NoOp,        NoOp,      NoOp,       NoOp],
        &[k(VolUp),   k(VolDown),   k(Kb1),   k(Kb2),     k(Space),      k(Kb6),    k(Kb7),             Trans,      Trans,    Trans,             Trans,       Trans,     NoOp,       NoOp],
    ],
];

// &[
//         &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,       NoOp,      NoOp,               NoOp,       NoOp,     NoOp,        NoOp,        NoOp,      NoOp,     NoOp],
//         &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,       NoOp,      NoOp,               NoOp,       NoOp,     NoOp,        NoOp,        NoOp,      NoOp,     NoOp],
//         &[Trans,      NoOp,         NoOp,     NoOp,       NoOp,       NoOp,      NoOp,               NoOp,       NoOp,     NoOp,        NoOp,        NoOp,      NoOp,     NoOp],
//         &[Trans,      Trans,        Trans,    Trans,      Trans,      Trans,     Trans,              Trans,      Trans,    Trans,       Trans,       Trans,     Trans,    Trans],
//     ],

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Layer {
    Default,
    Symbols,
    MacSymbols,
    Numbers,
    Navigation,
    Tabbing,
    Menu,
    CS,
    Missing,
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Default
    }
}

impl From<usize> for Layer {
    fn from(i: usize) -> Layer {
        match i {
            0 => Layer::Default,
            1 => Layer::Symbols,
            2 => Layer::MacSymbols,
            3 => Layer::Numbers,
            4 => Layer::Navigation,
            5 => Layer::Tabbing,
            6 => Layer::Menu,
            7 => Layer::CS,
            _ => Layer::Missing,
        }
    }
}

impl From<Layer> for usize {
    fn from(layer: Layer) -> Self {
        match layer {
            Layer::Default => 0,
            Layer::Symbols => 1,
            Layer::MacSymbols => 2,
            Layer::Numbers => 3,
            Layer::Navigation => 4,
            Layer::Tabbing => 5,
            Layer::Menu => 6,
            Layer::CS => 7,
            Layer::Missing => 8,
        }
    }
}

impl From<Layer> for &str {
    fn from(layer: Layer) -> &'static str {
        match layer {
            Layer::Default => "default",
            Layer::Numbers => "numbers",
            Layer::Navigation => "nav",
            Layer::Symbols => "symbols",
            Layer::MacSymbols => "mac symbols",
            Layer::Tabbing => "tabbing",
            Layer::Menu => "menu",
            Layer::CS => "CS",
            Layer::Missing => "missing",
        }
    }
}
