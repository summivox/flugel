pub use decimal::d128;
pub use extprim::i128::i128;
pub use extprim::u128::u128;

pub type CoordRaw = i64;
pub type Coord = d128;
pub type Angle = d128;
pub type Factor = d128;

pub type ApertureId = u32;
pub type MacroVarId = u32;
pub type MacroPrimitiveId = u8;

#[derive(Debug)]
pub enum Command<'input> {
    /// G04
    Comment(&'input str),

    /// FSLAX??Y??
    Format {
        /// integer digits in x coordinates
        xi: u8,
        /// fraction digits in x coordinates
        xf: u8,
        /// integer digits in y coordinates
        yi: u8,
        /// fraction digits in y coordinates
        yf: u8,
    },

    /// MOIN
    UnitInch,
    /// MOMM
    UnitMm,

    /// X???Y???I???J???D01/D02/D03
    Operation {
        x: Option<CoordRaw>,
        y: Option<CoordRaw>,
        i: Option<CoordRaw>,
        j: Option<CoordRaw>,
        d: OperationType,
    },

    /// SRX??Y??I??J??
    StepRepeat {
        x: u32,
        y: u32,
        i: Coord,
        j: Coord,
    },
    /// SR
    StepRepeatClear,

    /// G01
    Line,
    /// G02
    ArcCW,
    /// G03
    ArcCCW,
    /// G74
    ArcSingleQuadrant,
    /// G75
    ArcMultiQuadrant,


    /// G36
    RegionStart,
    /// G37
    RegionEnd,


    /// LPD
    PolarityDark,
    /// LPC
    PolarityClear,

    /// LMN
    MirrorN,
    /// LMX
    MirrorX,
    /// LMY
    MirrorY,
    /// LMXY
    MirrorXY,

    /// LR???
    Rotate(Angle),

    /// LS???
    Scale(Factor),


    /// D??? (number >= 10)
    ApertureSelect(ApertureId),

    /// ABD??? (number >= 10)
    ApertureBlock(ApertureId),
    /// AB
    ApertureBlockClear,

    /// AD???xxx,???X???X???
    ApertureDefine {
        id: ApertureId,
        macro_name: &'input str,
        args: Vec<Coord>,
    },


    ApertureMacro {
        name: &'input str,
        body: Vec<MacroStmt<'input>>
    }
}
pub use self::Command::*;

#[derive(Debug)]
pub enum MacroStmt<'input> {
    MacroComment(&'input str),
    Assign(MacroVarId, MacroExpr),
    Primitive(MacroPrimitiveId, Vec<MacroExpr>)
}
pub use self::MacroStmt::*;

#[derive(Debug)]
pub enum MacroExpr {
    Lit(Coord),
    Var(MacroVarId),
    Bin(BinOp, Box<MacroExpr>, Box<MacroExpr>),
    Neg(Box<MacroExpr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// type of operation
#[derive(Debug)]
pub enum OperationType {
    /// D01
    Interpolate = 1,
    /// D02
    Move = 2,
    /// D03
    Flash = 3,
}
pub use self::OperationType::*;