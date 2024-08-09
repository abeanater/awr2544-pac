#[doc = "Register `CSI2_COMPLEXIO_CFG1` reader"]
pub type R = crate::R<Csi2ComplexioCfg1Spec>;
#[doc = "Register `CSI2_COMPLEXIO_CFG1` writer"]
pub type W = crate::W<Csi2ComplexioCfg1Spec>;
#[doc = "2:0\\]
Position and order of the CLOCK lane. 0, 5, 6 and 7 are reserved. The clock lane is always present but can not be at the position 5 even if the COMPLEX IO consists of 5 lanes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockPosition {
    #[doc = "1: Clock lane is at the position 1."]
    Position1 = 1,
    #[doc = "2: Clock lane is at the position 2."]
    Position2 = 2,
    #[doc = "3: Clock lane is at the position 3."]
    Position3 = 3,
    #[doc = "4: Clock lane is at the position 4."]
    Position4 = 4,
}
impl From<ClockPosition> for u8 {
    #[inline(always)]
    fn from(variant: ClockPosition) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClockPosition {
    type Ux = u8;
}
impl crate::IsEnum for ClockPosition {}
#[doc = "Field `CLOCK_POSITION` reader - 2:0\\]
Position and order of the CLOCK lane. 0, 5, 6 and 7 are reserved. The clock lane is always present but can not be at the position 5 even if the COMPLEX IO consists of 5 lanes."]
pub type ClockPositionR = crate::FieldReader<ClockPosition>;
impl ClockPositionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClockPosition> {
        match self.bits {
            1 => Some(ClockPosition::Position1),
            2 => Some(ClockPosition::Position2),
            3 => Some(ClockPosition::Position3),
            4 => Some(ClockPosition::Position4),
            _ => None,
        }
    }
    #[doc = "Clock lane is at the position 1."]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == ClockPosition::Position1
    }
    #[doc = "Clock lane is at the position 2."]
    #[inline(always)]
    pub fn is_position_2(&self) -> bool {
        *self == ClockPosition::Position2
    }
    #[doc = "Clock lane is at the position 3."]
    #[inline(always)]
    pub fn is_position_3(&self) -> bool {
        *self == ClockPosition::Position3
    }
    #[doc = "Clock lane is at the position 4."]
    #[inline(always)]
    pub fn is_position_4(&self) -> bool {
        *self == ClockPosition::Position4
    }
}
#[doc = "Field `CLOCK_POSITION` writer - 2:0\\]
Position and order of the CLOCK lane. 0, 5, 6 and 7 are reserved. The clock lane is always present but can not be at the position 5 even if the COMPLEX IO consists of 5 lanes."]
pub type ClockPositionW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClockPosition>;
impl<'a, REG> ClockPositionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock lane is at the position 1."]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPosition::Position1)
    }
    #[doc = "Clock lane is at the position 2."]
    #[inline(always)]
    pub fn position_2(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPosition::Position2)
    }
    #[doc = "Clock lane is at the position 3."]
    #[inline(always)]
    pub fn position_3(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPosition::Position3)
    }
    #[doc = "Clock lane is at the position 4."]
    #[inline(always)]
    pub fn position_4(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPosition::Position4)
    }
}
#[doc = "3:3\\]
+/- differential pin order of CLOCK lane.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockPol {
    #[doc = "0: +/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    Plusminus = 0,
    #[doc = "1: -/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    Minusplus = 1,
}
impl From<ClockPol> for bool {
    #[inline(always)]
    fn from(variant: ClockPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCK_POL` reader - 3:3\\]
+/- differential pin order of CLOCK lane."]
pub type ClockPolR = crate::BitReader<ClockPol>;
impl ClockPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClockPol {
        match self.bits {
            false => ClockPol::Plusminus,
            true => ClockPol::Minusplus,
        }
    }
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn is_plusminus(&self) -> bool {
        *self == ClockPol::Plusminus
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn is_minusplus(&self) -> bool {
        *self == ClockPol::Minusplus
    }
}
#[doc = "Field `CLOCK_POL` writer - 3:3\\]
+/- differential pin order of CLOCK lane."]
pub type ClockPolW<'a, REG> = crate::BitWriter<'a, REG, ClockPol>;
impl<'a, REG> ClockPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn plusminus(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPol::Plusminus)
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn minusplus(self) -> &'a mut crate::W<REG> {
        self.variant(ClockPol::Minusplus)
    }
}
#[doc = "6:4\\]
Position and order of the DATA lane 1. The data lane 1 is always present.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Data1Position {
    #[doc = "1: Data lane 1 is at the position 1."]
    Position1 = 1,
    #[doc = "2: Data lane 1 is at the position 2."]
    Position2 = 2,
    #[doc = "3: Data lane 1 is at the position 3."]
    Position3 = 3,
    #[doc = "4: Data lane 1 is at the position 4."]
    Position4 = 4,
    #[doc = "5: Data lane 1 is at the position 5."]
    Position5 = 5,
}
impl From<Data1Position> for u8 {
    #[inline(always)]
    fn from(variant: Data1Position) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Data1Position {
    type Ux = u8;
}
impl crate::IsEnum for Data1Position {}
#[doc = "Field `DATA1_POSITION` reader - 6:4\\]
Position and order of the DATA lane 1. The data lane 1 is always present."]
pub type Data1PositionR = crate::FieldReader<Data1Position>;
impl Data1PositionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Data1Position> {
        match self.bits {
            1 => Some(Data1Position::Position1),
            2 => Some(Data1Position::Position2),
            3 => Some(Data1Position::Position3),
            4 => Some(Data1Position::Position4),
            5 => Some(Data1Position::Position5),
            _ => None,
        }
    }
    #[doc = "Data lane 1 is at the position 1."]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == Data1Position::Position1
    }
    #[doc = "Data lane 1 is at the position 2."]
    #[inline(always)]
    pub fn is_position_2(&self) -> bool {
        *self == Data1Position::Position2
    }
    #[doc = "Data lane 1 is at the position 3."]
    #[inline(always)]
    pub fn is_position_3(&self) -> bool {
        *self == Data1Position::Position3
    }
    #[doc = "Data lane 1 is at the position 4."]
    #[inline(always)]
    pub fn is_position_4(&self) -> bool {
        *self == Data1Position::Position4
    }
    #[doc = "Data lane 1 is at the position 5."]
    #[inline(always)]
    pub fn is_position_5(&self) -> bool {
        *self == Data1Position::Position5
    }
}
#[doc = "Field `DATA1_POSITION` writer - 6:4\\]
Position and order of the DATA lane 1. The data lane 1 is always present."]
pub type Data1PositionW<'a, REG> = crate::FieldWriter<'a, REG, 3, Data1Position>;
impl<'a, REG> Data1PositionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data lane 1 is at the position 1."]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Position::Position1)
    }
    #[doc = "Data lane 1 is at the position 2."]
    #[inline(always)]
    pub fn position_2(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Position::Position2)
    }
    #[doc = "Data lane 1 is at the position 3."]
    #[inline(always)]
    pub fn position_3(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Position::Position3)
    }
    #[doc = "Data lane 1 is at the position 4."]
    #[inline(always)]
    pub fn position_4(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Position::Position4)
    }
    #[doc = "Data lane 1 is at the position 5."]
    #[inline(always)]
    pub fn position_5(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Position::Position5)
    }
}
#[doc = "7:7\\]
+/- pin differential pin order of DATA lane 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data1Pol {
    #[doc = "0: +/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    Plusminus = 0,
    #[doc = "1: -/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    Minusplus = 1,
}
impl From<Data1Pol> for bool {
    #[inline(always)]
    fn from(variant: Data1Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA1_POL` reader - 7:7\\]
+/- pin differential pin order of DATA lane 1"]
pub type Data1PolR = crate::BitReader<Data1Pol>;
impl Data1PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data1Pol {
        match self.bits {
            false => Data1Pol::Plusminus,
            true => Data1Pol::Minusplus,
        }
    }
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn is_plusminus(&self) -> bool {
        *self == Data1Pol::Plusminus
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn is_minusplus(&self) -> bool {
        *self == Data1Pol::Minusplus
    }
}
#[doc = "Field `DATA1_POL` writer - 7:7\\]
+/- pin differential pin order of DATA lane 1"]
pub type Data1PolW<'a, REG> = crate::BitWriter<'a, REG, Data1Pol>;
impl<'a, REG> Data1PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn plusminus(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Pol::Plusminus)
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn minusplus(self) -> &'a mut crate::W<REG> {
        self.variant(Data1Pol::Minusplus)
    }
}
#[doc = "Field `RESERVED1` reader - 8:8\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 8:8\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
Position and order of the DATA lane 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data2Position {
    #[doc = "0: Not used/connected"]
    NotUsed = 0,
    #[doc = "1: Data lane 2 is at the position 1."]
    Position1 = 1,
}
impl From<Data2Position> for bool {
    #[inline(always)]
    fn from(variant: Data2Position) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA2_POSITION` reader - 8:8\\]
Position and order of the DATA lane 2."]
pub type Data2PositionR = crate::BitReader<Data2Position>;
impl Data2PositionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data2Position {
        match self.bits {
            false => Data2Position::NotUsed,
            true => Data2Position::Position1,
        }
    }
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Data2Position::NotUsed
    }
    #[doc = "Data lane 2 is at the position 1."]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == Data2Position::Position1
    }
}
#[doc = "Field `DATA2_POSITION` writer - 8:8\\]
Position and order of the DATA lane 2."]
pub type Data2PositionW<'a, REG> = crate::BitWriter<'a, REG, Data2Position>;
impl<'a, REG> Data2PositionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Data2Position::NotUsed)
    }
    #[doc = "Data lane 2 is at the position 1."]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut crate::W<REG> {
        self.variant(Data2Position::Position1)
    }
}
#[doc = "Field `RESERVED2` reader - 11:11\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 11:11\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "11:11\\]
+/- differential pin order of DATA lane 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data2Pol {
    #[doc = "0: +/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    Plusminus = 0,
    #[doc = "1: -/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    Minusplus = 1,
}
impl From<Data2Pol> for bool {
    #[inline(always)]
    fn from(variant: Data2Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA2_POL` reader - 11:11\\]
+/- differential pin order of DATA lane 2."]
pub type Data2PolR = crate::BitReader<Data2Pol>;
impl Data2PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data2Pol {
        match self.bits {
            false => Data2Pol::Plusminus,
            true => Data2Pol::Minusplus,
        }
    }
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn is_plusminus(&self) -> bool {
        *self == Data2Pol::Plusminus
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn is_minusplus(&self) -> bool {
        *self == Data2Pol::Minusplus
    }
}
#[doc = "Field `DATA2_POL` writer - 11:11\\]
+/- differential pin order of DATA lane 2."]
pub type Data2PolW<'a, REG> = crate::BitWriter<'a, REG, Data2Pol>;
impl<'a, REG> Data2PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn plusminus(self) -> &'a mut crate::W<REG> {
        self.variant(Data2Pol::Plusminus)
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn minusplus(self) -> &'a mut crate::W<REG> {
        self.variant(Data2Pol::Minusplus)
    }
}
#[doc = "Field `RESERVED3` reader - 12:12\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 12:12\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "12:12\\]
Position and order of the DATA lane 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data3Position {
    #[doc = "0: Not used/connected"]
    NotUsed = 0,
    #[doc = "1: Data lane 3 is at the position 1."]
    Position1 = 1,
}
impl From<Data3Position> for bool {
    #[inline(always)]
    fn from(variant: Data3Position) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA3_POSITION` reader - 12:12\\]
Position and order of the DATA lane 3."]
pub type Data3PositionR = crate::BitReader<Data3Position>;
impl Data3PositionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data3Position {
        match self.bits {
            false => Data3Position::NotUsed,
            true => Data3Position::Position1,
        }
    }
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Data3Position::NotUsed
    }
    #[doc = "Data lane 3 is at the position 1."]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == Data3Position::Position1
    }
}
#[doc = "Field `DATA3_POSITION` writer - 12:12\\]
Position and order of the DATA lane 3."]
pub type Data3PositionW<'a, REG> = crate::BitWriter<'a, REG, Data3Position>;
impl<'a, REG> Data3PositionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Data3Position::NotUsed)
    }
    #[doc = "Data lane 3 is at the position 1."]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut crate::W<REG> {
        self.variant(Data3Position::Position1)
    }
}
#[doc = "Field `RESERVED4` reader - 15:15\\]
Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 15:15\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "15:15\\]
+/- differential pin order of DATA lane 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data3Pol {
    #[doc = "0: +/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    Plusminus = 0,
    #[doc = "1: -/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    Minusplus = 1,
}
impl From<Data3Pol> for bool {
    #[inline(always)]
    fn from(variant: Data3Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA3_POL` reader - 15:15\\]
+/- differential pin order of DATA lane 3."]
pub type Data3PolR = crate::BitReader<Data3Pol>;
impl Data3PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data3Pol {
        match self.bits {
            false => Data3Pol::Plusminus,
            true => Data3Pol::Minusplus,
        }
    }
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn is_plusminus(&self) -> bool {
        *self == Data3Pol::Plusminus
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn is_minusplus(&self) -> bool {
        *self == Data3Pol::Minusplus
    }
}
#[doc = "Field `DATA3_POL` writer - 15:15\\]
+/- differential pin order of DATA lane 3."]
pub type Data3PolW<'a, REG> = crate::BitWriter<'a, REG, Data3Pol>;
impl<'a, REG> Data3PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn plusminus(self) -> &'a mut crate::W<REG> {
        self.variant(Data3Pol::Plusminus)
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn minusplus(self) -> &'a mut crate::W<REG> {
        self.variant(Data3Pol::Minusplus)
    }
}
#[doc = "Field `RESERVED5` reader - 16:16\\]
Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RESERVED5` writer - 16:16\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "16:16\\]
Position and order of the DATA lane 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data4Position {
    #[doc = "0: Not used/connected"]
    NotUsed = 0,
    #[doc = "1: Data lane 4 is at the position 1."]
    Position1 = 1,
}
impl From<Data4Position> for bool {
    #[inline(always)]
    fn from(variant: Data4Position) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA4_POSITION` reader - 16:16\\]
Position and order of the DATA lane 4."]
pub type Data4PositionR = crate::BitReader<Data4Position>;
impl Data4PositionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data4Position {
        match self.bits {
            false => Data4Position::NotUsed,
            true => Data4Position::Position1,
        }
    }
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Data4Position::NotUsed
    }
    #[doc = "Data lane 4 is at the position 1."]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == Data4Position::Position1
    }
}
#[doc = "Field `DATA4_POSITION` writer - 16:16\\]
Position and order of the DATA lane 4."]
pub type Data4PositionW<'a, REG> = crate::BitWriter<'a, REG, Data4Position>;
impl<'a, REG> Data4PositionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used/connected"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Data4Position::NotUsed)
    }
    #[doc = "Data lane 4 is at the position 1."]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut crate::W<REG> {
        self.variant(Data4Position::Position1)
    }
}
#[doc = "Field `RESERVED6` reader - 19:19\\]
Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - 19:19\\]
Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "19:19\\]
+/- differential pin order of DATA lane 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data4Pol {
    #[doc = "0: +/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    Plusminus = 0,
    #[doc = "1: -/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    Minusplus = 1,
}
impl From<Data4Pol> for bool {
    #[inline(always)]
    fn from(variant: Data4Pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA4_POL` reader - 19:19\\]
+/- differential pin order of DATA lane 4."]
pub type Data4PolR = crate::BitReader<Data4Pol>;
impl Data4PolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data4Pol {
        match self.bits {
            false => Data4Pol::Plusminus,
            true => Data4Pol::Minusplus,
        }
    }
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn is_plusminus(&self) -> bool {
        *self == Data4Pol::Plusminus
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn is_minusplus(&self) -> bool {
        *self == Data4Pol::Minusplus
    }
}
#[doc = "Field `DATA4_POL` writer - 19:19\\]
+/- differential pin order of DATA lane 4."]
pub type Data4PolW<'a, REG> = crate::BitWriter<'a, REG, Data4Pol>;
impl<'a, REG> Data4PolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "+/- pin order (CSI2.DX=+ and CSI2.DY=-)"]
    #[inline(always)]
    pub fn plusminus(self) -> &'a mut crate::W<REG> {
        self.variant(Data4Pol::Plusminus)
    }
    #[doc = "-/+ pin order (CSI2.DX=- and CSI2.DY=+)"]
    #[inline(always)]
    pub fn minusplus(self) -> &'a mut crate::W<REG> {
        self.variant(Data4Pol::Minusplus)
    }
}
#[doc = "20:20\\]
Select the external LDO for the CSI2PHY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseLdoExternal {
    #[doc = "0: CSI2PHY internal LDO is used."]
    Internalldo = 0,
    #[doc = "1: External LDO is used. CSI2PHY LDO is tri-stated."]
    Externalldo = 1,
}
impl From<UseLdoExternal> for bool {
    #[inline(always)]
    fn from(variant: UseLdoExternal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_LDO_EXTERNAL` reader - 20:20\\]
Select the external LDO for the CSI2PHY."]
pub type UseLdoExternalR = crate::BitReader<UseLdoExternal>;
impl UseLdoExternalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseLdoExternal {
        match self.bits {
            false => UseLdoExternal::Internalldo,
            true => UseLdoExternal::Externalldo,
        }
    }
    #[doc = "CSI2PHY internal LDO is used."]
    #[inline(always)]
    pub fn is_internalldo(&self) -> bool {
        *self == UseLdoExternal::Internalldo
    }
    #[doc = "External LDO is used. CSI2PHY LDO is tri-stated."]
    #[inline(always)]
    pub fn is_externalldo(&self) -> bool {
        *self == UseLdoExternal::Externalldo
    }
}
#[doc = "Field `USE_LDO_EXTERNAL` writer - 20:20\\]
Select the external LDO for the CSI2PHY."]
pub type UseLdoExternalW<'a, REG> = crate::BitWriter<'a, REG, UseLdoExternal>;
impl<'a, REG> UseLdoExternalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI2PHY internal LDO is used."]
    #[inline(always)]
    pub fn internalldo(self) -> &'a mut crate::W<REG> {
        self.variant(UseLdoExternal::Internalldo)
    }
    #[doc = "External LDO is used. CSI2PHY LDO is tri-stated."]
    #[inline(always)]
    pub fn externalldo(self) -> &'a mut crate::W<REG> {
        self.variant(UseLdoExternal::Externalldo)
    }
}
#[doc = "21:21\\]
Indicates the state of the signal LDOPWRGOOD. VDDALDOCSI2PLL: 1.2V power supply for the PLL. The voltage is supplied by the internal or external LDO. The interrupt LDO_POWER_GOOD_IRQ is generated when a transition is detected on the signal LDOPWRGOOD from the CSI2PHY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdoPowerGoodState {
    #[doc = "0: VDDALDOCSI2PLL power supply is down"]
    Supplydown = 0,
    #[doc = "1: VDDALDOCSI2PLL power supply is up"]
    Supplyup = 1,
}
impl From<LdoPowerGoodState> for bool {
    #[inline(always)]
    fn from(variant: LdoPowerGoodState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDO_POWER_GOOD_STATE` reader - 21:21\\]
Indicates the state of the signal LDOPWRGOOD. VDDALDOCSI2PLL: 1.2V power supply for the PLL. The voltage is supplied by the internal or external LDO. The interrupt LDO_POWER_GOOD_IRQ is generated when a transition is detected on the signal LDOPWRGOOD from the CSI2PHY."]
pub type LdoPowerGoodStateR = crate::BitReader<LdoPowerGoodState>;
impl LdoPowerGoodStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdoPowerGoodState {
        match self.bits {
            false => LdoPowerGoodState::Supplydown,
            true => LdoPowerGoodState::Supplyup,
        }
    }
    #[doc = "VDDALDOCSI2PLL power supply is down"]
    #[inline(always)]
    pub fn is_supplydown(&self) -> bool {
        *self == LdoPowerGoodState::Supplydown
    }
    #[doc = "VDDALDOCSI2PLL power supply is up"]
    #[inline(always)]
    pub fn is_supplyup(&self) -> bool {
        *self == LdoPowerGoodState::Supplyup
    }
}
#[doc = "Field `LDO_POWER_GOOD_STATE` writer - 21:21\\]
Indicates the state of the signal LDOPWRGOOD. VDDALDOCSI2PLL: 1.2V power supply for the PLL. The voltage is supplied by the internal or external LDO. The interrupt LDO_POWER_GOOD_IRQ is generated when a transition is detected on the signal LDOPWRGOOD from the CSI2PHY."]
pub type LdoPowerGoodStateW<'a, REG> = crate::BitWriter<'a, REG, LdoPowerGoodState>;
impl<'a, REG> LdoPowerGoodStateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDALDOCSI2PLL power supply is down"]
    #[inline(always)]
    pub fn supplydown(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodState::Supplydown)
    }
    #[doc = "VDDALDOCSI2PLL power supply is up"]
    #[inline(always)]
    pub fn supplyup(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodState::Supplyup)
    }
}
#[doc = "26:25\\]
Status of the power control of the complex IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrStatus {
    #[doc = "0: Complex IO in OFF state"]
    StateOff = 0,
    #[doc = "1: Complex IO in ON state"]
    StateOn = 1,
    #[doc = "2: Complex IO in Ultra Low Power state"]
    StateUlp = 2,
}
impl From<PwrStatus> for u8 {
    #[inline(always)]
    fn from(variant: PwrStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrStatus {
    type Ux = u8;
}
impl crate::IsEnum for PwrStatus {}
#[doc = "Field `PWR_STATUS` reader - 26:25\\]
Status of the power control of the complex IO"]
pub type PwrStatusR = crate::FieldReader<PwrStatus>;
impl PwrStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrStatus> {
        match self.bits {
            0 => Some(PwrStatus::StateOff),
            1 => Some(PwrStatus::StateOn),
            2 => Some(PwrStatus::StateUlp),
            _ => None,
        }
    }
    #[doc = "Complex IO in OFF state"]
    #[inline(always)]
    pub fn is_state_off(&self) -> bool {
        *self == PwrStatus::StateOff
    }
    #[doc = "Complex IO in ON state"]
    #[inline(always)]
    pub fn is_state_on(&self) -> bool {
        *self == PwrStatus::StateOn
    }
    #[doc = "Complex IO in Ultra Low Power state"]
    #[inline(always)]
    pub fn is_state_ulp(&self) -> bool {
        *self == PwrStatus::StateUlp
    }
}
#[doc = "Field `PWR_STATUS` writer - 26:25\\]
Status of the power control of the complex IO"]
pub type PwrStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrStatus>;
impl<'a, REG> PwrStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Complex IO in OFF state"]
    #[inline(always)]
    pub fn state_off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStatus::StateOff)
    }
    #[doc = "Complex IO in ON state"]
    #[inline(always)]
    pub fn state_on(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStatus::StateOn)
    }
    #[doc = "Complex IO in Ultra Low Power state"]
    #[inline(always)]
    pub fn state_ulp(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStatus::StateUlp)
    }
}
#[doc = "28:27\\]
Command for power control of the complex IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrCmd {
    #[doc = "0: Command to change to OFF state"]
    StateOff = 0,
    #[doc = "1: Command to change to ON state"]
    StateOn = 1,
    #[doc = "2: Command to change to Ultra Low Power state"]
    StateUlp = 2,
}
impl From<PwrCmd> for u8 {
    #[inline(always)]
    fn from(variant: PwrCmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrCmd {
    type Ux = u8;
}
impl crate::IsEnum for PwrCmd {}
#[doc = "Field `PWR_CMD` reader - 28:27\\]
Command for power control of the complex IO"]
pub type PwrCmdR = crate::FieldReader<PwrCmd>;
impl PwrCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrCmd> {
        match self.bits {
            0 => Some(PwrCmd::StateOff),
            1 => Some(PwrCmd::StateOn),
            2 => Some(PwrCmd::StateUlp),
            _ => None,
        }
    }
    #[doc = "Command to change to OFF state"]
    #[inline(always)]
    pub fn is_state_off(&self) -> bool {
        *self == PwrCmd::StateOff
    }
    #[doc = "Command to change to ON state"]
    #[inline(always)]
    pub fn is_state_on(&self) -> bool {
        *self == PwrCmd::StateOn
    }
    #[doc = "Command to change to Ultra Low Power state"]
    #[inline(always)]
    pub fn is_state_ulp(&self) -> bool {
        *self == PwrCmd::StateUlp
    }
}
#[doc = "Field `PWR_CMD` writer - 28:27\\]
Command for power control of the complex IO"]
pub type PwrCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrCmd>;
impl<'a, REG> PwrCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command to change to OFF state"]
    #[inline(always)]
    pub fn state_off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCmd::StateOff)
    }
    #[doc = "Command to change to ON state"]
    #[inline(always)]
    pub fn state_on(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCmd::StateOn)
    }
    #[doc = "Command to change to Ultra Low Power state"]
    #[inline(always)]
    pub fn state_ulp(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCmd::StateUlp)
    }
}
#[doc = "29:29\\]
Internal reset monitoring of the power domain using the PPI byte clock from the complex IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetDone {
    #[doc = "0: Internal module reset is on going."]
    Resetongoing = 0,
    #[doc = "1: Reset completed."]
    Resetcompleted = 1,
}
impl From<ResetDone> for bool {
    #[inline(always)]
    fn from(variant: ResetDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_DONE` reader - 29:29\\]
Internal reset monitoring of the power domain using the PPI byte clock from the complex IO"]
pub type ResetDoneR = crate::BitReader<ResetDone>;
impl ResetDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetDone {
        match self.bits {
            false => ResetDone::Resetongoing,
            true => ResetDone::Resetcompleted,
        }
    }
    #[doc = "Internal module reset is on going."]
    #[inline(always)]
    pub fn is_resetongoing(&self) -> bool {
        *self == ResetDone::Resetongoing
    }
    #[doc = "Reset completed."]
    #[inline(always)]
    pub fn is_resetcompleted(&self) -> bool {
        *self == ResetDone::Resetcompleted
    }
}
#[doc = "Field `RESET_DONE` writer - 29:29\\]
Internal reset monitoring of the power domain using the PPI byte clock from the complex IO"]
pub type ResetDoneW<'a, REG> = crate::BitWriter<'a, REG, ResetDone>;
impl<'a, REG> ResetDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module reset is on going."]
    #[inline(always)]
    pub fn resetongoing(self) -> &'a mut crate::W<REG> {
        self.variant(ResetDone::Resetongoing)
    }
    #[doc = "Reset completed."]
    #[inline(always)]
    pub fn resetcompleted(self) -> &'a mut crate::W<REG> {
        self.variant(ResetDone::Resetcompleted)
    }
}
#[doc = "30:30\\]
Allows the synchronized update of the shadow registers when the signal DISPCUpdateSync is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gobit {
    #[doc = "0: Resets the GOBIT. The hardware has finished the update of the shadow SCP registers. The bit is reset by Hardware. The SW can reset the bit in case the user decides to abort it. There is no guarantee that the SW reset is done before the transfer of the values to the complex IO."]
    Reset = 0,
    #[doc = "1: Set the GOBIT. Only when the transfer of the new values for the three first registers is completed (3, 2, 1, or 0 transfers are performed based on the number of registers to update), the GOBIT is reset. The DISPCUpdateSync signal is used to synchronize the update. The bit shall be set only when it is in reset state."]
    Set = 1,
}
impl From<Gobit> for bool {
    #[inline(always)]
    fn from(variant: Gobit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GOBIT` reader - 30:30\\]
Allows the synchronized update of the shadow registers when the signal DISPCUpdateSync is active."]
pub type GobitR = crate::BitReader<Gobit>;
impl GobitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gobit {
        match self.bits {
            false => Gobit::Reset,
            true => Gobit::Set,
        }
    }
    #[doc = "Resets the GOBIT. The hardware has finished the update of the shadow SCP registers. The bit is reset by Hardware. The SW can reset the bit in case the user decides to abort it. There is no guarantee that the SW reset is done before the transfer of the values to the complex IO."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gobit::Reset
    }
    #[doc = "Set the GOBIT. Only when the transfer of the new values for the three first registers is completed (3, 2, 1, or 0 transfers are performed based on the number of registers to update), the GOBIT is reset. The DISPCUpdateSync signal is used to synchronize the update. The bit shall be set only when it is in reset state."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gobit::Set
    }
}
#[doc = "Field `GOBIT` writer - 30:30\\]
Allows the synchronized update of the shadow registers when the signal DISPCUpdateSync is active."]
pub type GobitW<'a, REG> = crate::BitWriter<'a, REG, Gobit>;
impl<'a, REG> GobitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the GOBIT. The hardware has finished the update of the shadow SCP registers. The bit is reset by Hardware. The SW can reset the bit in case the user decides to abort it. There is no guarantee that the SW reset is done before the transfer of the values to the complex IO."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Gobit::Reset)
    }
    #[doc = "Set the GOBIT. Only when the transfer of the new values for the three first registers is completed (3, 2, 1, or 0 transfers are performed based on the number of registers to update), the GOBIT is reset. The DISPCUpdateSync signal is used to synchronize the update. The bit shall be set only when it is in reset state."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Gobit::Set)
    }
}
#[doc = "31:31\\]
Shadowing configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shadowing {
    #[doc = "0: Disabled. The writes to the first 3 registers of the complex IO address map is done like the other SCP registers."]
    Disable = 0,
    #[doc = "1: Enabled. The writes to the first 3 registers of the complex IO address map is done only when the GObit is set and when the signal DISPCUpdataSync from the display controller module is active."]
    Enable = 1,
}
impl From<Shadowing> for bool {
    #[inline(always)]
    fn from(variant: Shadowing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHADOWING` reader - 31:31\\]
Shadowing configuration."]
pub type ShadowingR = crate::BitReader<Shadowing>;
impl ShadowingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shadowing {
        match self.bits {
            false => Shadowing::Disable,
            true => Shadowing::Enable,
        }
    }
    #[doc = "Disabled. The writes to the first 3 registers of the complex IO address map is done like the other SCP registers."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Shadowing::Disable
    }
    #[doc = "Enabled. The writes to the first 3 registers of the complex IO address map is done only when the GObit is set and when the signal DISPCUpdataSync from the display controller module is active."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Shadowing::Enable
    }
}
#[doc = "Field `SHADOWING` writer - 31:31\\]
Shadowing configuration."]
pub type ShadowingW<'a, REG> = crate::BitWriter<'a, REG, Shadowing>;
impl<'a, REG> ShadowingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The writes to the first 3 registers of the complex IO address map is done like the other SCP registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Shadowing::Disable)
    }
    #[doc = "Enabled. The writes to the first 3 registers of the complex IO address map is done only when the GObit is set and when the signal DISPCUpdataSync from the display controller module is active."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Shadowing::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Position and order of the CLOCK lane. 0, 5, 6 and 7 are reserved. The clock lane is always present but can not be at the position 5 even if the COMPLEX IO consists of 5 lanes."]
    #[inline(always)]
    pub fn clock_position(&self) -> ClockPositionR {
        ClockPositionR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
+/- differential pin order of CLOCK lane."]
    #[inline(always)]
    pub fn clock_pol(&self) -> ClockPolR {
        ClockPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Position and order of the DATA lane 1. The data lane 1 is always present."]
    #[inline(always)]
    pub fn data1_position(&self) -> Data1PositionR {
        Data1PositionR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
+/- pin differential pin order of DATA lane 1"]
    #[inline(always)]
    pub fn data1_pol(&self) -> Data1PolR {
        Data1PolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Position and order of the DATA lane 2."]
    #[inline(always)]
    pub fn data2_position(&self) -> Data2PositionR {
        Data2PositionR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
+/- differential pin order of DATA lane 2."]
    #[inline(always)]
    pub fn data2_pol(&self) -> Data2PolR {
        Data2PolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Position and order of the DATA lane 3."]
    #[inline(always)]
    pub fn data3_position(&self) -> Data3PositionR {
        Data3PositionR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
+/- differential pin order of DATA lane 3."]
    #[inline(always)]
    pub fn data3_pol(&self) -> Data3PolR {
        Data3PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Position and order of the DATA lane 4."]
    #[inline(always)]
    pub fn data4_position(&self) -> Data4PositionR {
        Data4PositionR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
+/- differential pin order of DATA lane 4."]
    #[inline(always)]
    pub fn data4_pol(&self) -> Data4PolR {
        Data4PolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Select the external LDO for the CSI2PHY."]
    #[inline(always)]
    pub fn use_ldo_external(&self) -> UseLdoExternalR {
        UseLdoExternalR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Indicates the state of the signal LDOPWRGOOD. VDDALDOCSI2PLL: 1.2V power supply for the PLL. The voltage is supplied by the internal or external LDO. The interrupt LDO_POWER_GOOD_IRQ is generated when a transition is detected on the signal LDOPWRGOOD from the CSI2PHY."]
    #[inline(always)]
    pub fn ldo_power_good_state(&self) -> LdoPowerGoodStateR {
        LdoPowerGoodStateR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Status of the power control of the complex IO"]
    #[inline(always)]
    pub fn pwr_status(&self) -> PwrStatusR {
        PwrStatusR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Command for power control of the complex IO"]
    #[inline(always)]
    pub fn pwr_cmd(&self) -> PwrCmdR {
        PwrCmdR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal reset monitoring of the power domain using the PPI byte clock from the complex IO"]
    #[inline(always)]
    pub fn reset_done(&self) -> ResetDoneR {
        ResetDoneR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Allows the synchronized update of the shadow registers when the signal DISPCUpdateSync is active."]
    #[inline(always)]
    pub fn gobit(&self) -> GobitR {
        GobitR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Shadowing configuration."]
    #[inline(always)]
    pub fn shadowing(&self) -> ShadowingR {
        ShadowingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Position and order of the CLOCK lane. 0, 5, 6 and 7 are reserved. The clock lane is always present but can not be at the position 5 even if the COMPLEX IO consists of 5 lanes."]
    #[inline(always)]
    #[must_use]
    pub fn clock_position(&mut self) -> ClockPositionW<Csi2ComplexioCfg1Spec> {
        ClockPositionW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
+/- differential pin order of CLOCK lane."]
    #[inline(always)]
    #[must_use]
    pub fn clock_pol(&mut self) -> ClockPolW<Csi2ComplexioCfg1Spec> {
        ClockPolW::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Position and order of the DATA lane 1. The data lane 1 is always present."]
    #[inline(always)]
    #[must_use]
    pub fn data1_position(&mut self) -> Data1PositionW<Csi2ComplexioCfg1Spec> {
        Data1PositionW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
+/- pin differential pin order of DATA lane 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1_pol(&mut self) -> Data1PolW<Csi2ComplexioCfg1Spec> {
        Data1PolW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2ComplexioCfg1Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 8 - 8:8\\]
Position and order of the DATA lane 2."]
    #[inline(always)]
    #[must_use]
    pub fn data2_position(&mut self) -> Data2PositionW<Csi2ComplexioCfg1Spec> {
        Data2PositionW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2ComplexioCfg1Spec> {
        Reserved2W::new(self, 11)
    }
    #[doc = "Bit 11 - 11:11\\]
+/- differential pin order of DATA lane 2."]
    #[inline(always)]
    #[must_use]
    pub fn data2_pol(&mut self) -> Data2PolW<Csi2ComplexioCfg1Spec> {
        Data2PolW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2ComplexioCfg1Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 12 - 12:12\\]
Position and order of the DATA lane 3."]
    #[inline(always)]
    #[must_use]
    pub fn data3_position(&mut self) -> Data3PositionW<Csi2ComplexioCfg1Spec> {
        Data3PositionW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Csi2ComplexioCfg1Spec> {
        Reserved4W::new(self, 15)
    }
    #[doc = "Bit 15 - 15:15\\]
+/- differential pin order of DATA lane 3."]
    #[inline(always)]
    #[must_use]
    pub fn data3_pol(&mut self) -> Data3PolW<Csi2ComplexioCfg1Spec> {
        Data3PolW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Csi2ComplexioCfg1Spec> {
        Reserved5W::new(self, 16)
    }
    #[doc = "Bit 16 - 16:16\\]
Position and order of the DATA lane 4."]
    #[inline(always)]
    #[must_use]
    pub fn data4_position(&mut self) -> Data4PositionW<Csi2ComplexioCfg1Spec> {
        Data4PositionW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Csi2ComplexioCfg1Spec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 19 - 19:19\\]
+/- differential pin order of DATA lane 4."]
    #[inline(always)]
    #[must_use]
    pub fn data4_pol(&mut self) -> Data4PolW<Csi2ComplexioCfg1Spec> {
        Data4PolW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Select the external LDO for the CSI2PHY."]
    #[inline(always)]
    #[must_use]
    pub fn use_ldo_external(&mut self) -> UseLdoExternalW<Csi2ComplexioCfg1Spec> {
        UseLdoExternalW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Indicates the state of the signal LDOPWRGOOD. VDDALDOCSI2PLL: 1.2V power supply for the PLL. The voltage is supplied by the internal or external LDO. The interrupt LDO_POWER_GOOD_IRQ is generated when a transition is detected on the signal LDOPWRGOOD from the CSI2PHY."]
    #[inline(always)]
    #[must_use]
    pub fn ldo_power_good_state(&mut self) -> LdoPowerGoodStateW<Csi2ComplexioCfg1Spec> {
        LdoPowerGoodStateW::new(self, 21)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Status of the power control of the complex IO"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_status(&mut self) -> PwrStatusW<Csi2ComplexioCfg1Spec> {
        PwrStatusW::new(self, 25)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Command for power control of the complex IO"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_cmd(&mut self) -> PwrCmdW<Csi2ComplexioCfg1Spec> {
        PwrCmdW::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal reset monitoring of the power domain using the PPI byte clock from the complex IO"]
    #[inline(always)]
    #[must_use]
    pub fn reset_done(&mut self) -> ResetDoneW<Csi2ComplexioCfg1Spec> {
        ResetDoneW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Allows the synchronized update of the shadow registers when the signal DISPCUpdateSync is active."]
    #[inline(always)]
    #[must_use]
    pub fn gobit(&mut self) -> GobitW<Csi2ComplexioCfg1Spec> {
        GobitW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Shadowing configuration."]
    #[inline(always)]
    #[must_use]
    pub fn shadowing(&mut self) -> ShadowingW<Csi2ComplexioCfg1Spec> {
        ShadowingW::new(self, 31)
    }
}
#[doc = "COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the order and position of the lanes (clock and data) and the polarity order for the control of the PHY differential signals in addition to the control bit for the power FSM.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ComplexioCfg1Spec;
impl crate::RegisterSpec for Csi2ComplexioCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_complexio_cfg1::R`](R) reader structure"]
impl crate::Readable for Csi2ComplexioCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_complexio_cfg1::W`](W) writer structure"]
impl crate::Writable for Csi2ComplexioCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_COMPLEXIO_CFG1 to value 0"]
impl crate::Resettable for Csi2ComplexioCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
