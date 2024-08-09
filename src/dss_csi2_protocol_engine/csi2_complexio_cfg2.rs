#[doc = "Register `CSI2_COMPLEXIO_CFG2` reader"]
pub type R = crate::R<Csi2ComplexioCfg2Spec>;
#[doc = "Register `CSI2_COMPLEXIO_CFG2` writer"]
pub type W = crate::W<Csi2ComplexioCfg2Spec>;
#[doc = "0:0\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane1UlpsSig1 {
    #[doc = "0: READ:Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    Active = 1,
}
impl From<Lane1UlpsSig1> for bool {
    #[inline(always)]
    fn from(variant: Lane1UlpsSig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE1_ULPS_SIG1` reader - 0:0\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane1UlpsSig1R = crate::BitReader<Lane1UlpsSig1>;
impl Lane1UlpsSig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane1UlpsSig1 {
        match self.bits {
            false => Lane1UlpsSig1::Inactive,
            true => Lane1UlpsSig1::Active,
        }
    }
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane1UlpsSig1::Inactive
    }
    #[doc = "READ:Active state effective WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane1UlpsSig1::Active
    }
}
#[doc = "Field `LANE1_ULPS_SIG1` writer - 0:0\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane1UlpsSig1W<'a, REG> = crate::BitWriter<'a, REG, Lane1UlpsSig1>;
impl<'a, REG> Lane1UlpsSig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1UlpsSig1::Inactive)
    }
    #[doc = "READ:Active state effective WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1UlpsSig1::Active)
    }
}
#[doc = "1:1\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane2UlpsSig1 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    Active = 1,
}
impl From<Lane2UlpsSig1> for bool {
    #[inline(always)]
    fn from(variant: Lane2UlpsSig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE2_ULPS_SIG1` reader - 1:1\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane2UlpsSig1R = crate::BitReader<Lane2UlpsSig1>;
impl Lane2UlpsSig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane2UlpsSig1 {
        match self.bits {
            false => Lane2UlpsSig1::Inactive,
            true => Lane2UlpsSig1::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane2UlpsSig1::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane2UlpsSig1::Active
    }
}
#[doc = "Field `LANE2_ULPS_SIG1` writer - 1:1\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane2UlpsSig1W<'a, REG> = crate::BitWriter<'a, REG, Lane2UlpsSig1>;
impl<'a, REG> Lane2UlpsSig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2UlpsSig1::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2UlpsSig1::Active)
    }
}
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "2:2\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane3UlpsSig1 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    Active = 1,
}
impl From<Lane3UlpsSig1> for bool {
    #[inline(always)]
    fn from(variant: Lane3UlpsSig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE3_ULPS_SIG1` reader - 2:2\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane3UlpsSig1R = crate::BitReader<Lane3UlpsSig1>;
impl Lane3UlpsSig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane3UlpsSig1 {
        match self.bits {
            false => Lane3UlpsSig1::Inactive,
            true => Lane3UlpsSig1::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane3UlpsSig1::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane3UlpsSig1::Active
    }
}
#[doc = "Field `LANE3_ULPS_SIG1` writer - 2:2\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane3UlpsSig1W<'a, REG> = crate::BitWriter<'a, REG, Lane3UlpsSig1>;
impl<'a, REG> Lane3UlpsSig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3UlpsSig1::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3UlpsSig1::Active)
    }
}
#[doc = "Field `RESERVED2` reader - "]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "3:3\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane4UlpsSig1 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    Active = 1,
}
impl From<Lane4UlpsSig1> for bool {
    #[inline(always)]
    fn from(variant: Lane4UlpsSig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE4_ULPS_SIG1` reader - 3:3\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane4UlpsSig1R = crate::BitReader<Lane4UlpsSig1>;
impl Lane4UlpsSig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane4UlpsSig1 {
        match self.bits {
            false => Lane4UlpsSig1::Inactive,
            true => Lane4UlpsSig1::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane4UlpsSig1::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane4UlpsSig1::Active
    }
}
#[doc = "Field `LANE4_ULPS_SIG1` writer - 3:3\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane4UlpsSig1W<'a, REG> = crate::BitWriter<'a, REG, Lane4UlpsSig1>;
impl<'a, REG> Lane4UlpsSig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane4UlpsSig1::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane4UlpsSig1::Active)
    }
}
#[doc = "Field `RESERVED3` reader - 4:4\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 4:4\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane5UlpsSig1 {
    #[doc = "0: READ:Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    Active = 1,
}
impl From<Lane5UlpsSig1> for bool {
    #[inline(always)]
    fn from(variant: Lane5UlpsSig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE5_ULPS_SIG1` reader - 4:4\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane5UlpsSig1R = crate::BitReader<Lane5UlpsSig1>;
impl Lane5UlpsSig1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane5UlpsSig1 {
        match self.bits {
            false => Lane5UlpsSig1::Inactive,
            true => Lane5UlpsSig1::Active,
        }
    }
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane5UlpsSig1::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane5UlpsSig1::Active
    }
}
#[doc = "Field `LANE5_ULPS_SIG1` writer - 4:4\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane5UlpsSig1W<'a, REG> = crate::BitWriter<'a, REG, Lane5UlpsSig1>;
impl<'a, REG> Lane5UlpsSig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane5UlpsSig1::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane5UlpsSig1::Active)
    }
}
#[doc = "5:5\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #1 is a data lane. The state of the signal TxUlpsClk is changed if lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane1UlpsSig2 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    Active = 1,
}
impl From<Lane1UlpsSig2> for bool {
    #[inline(always)]
    fn from(variant: Lane1UlpsSig2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE1_ULPS_SIG2` reader - 5:5\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #1 is a data lane. The state of the signal TxUlpsClk is changed if lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane1UlpsSig2R = crate::BitReader<Lane1UlpsSig2>;
impl Lane1UlpsSig2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane1UlpsSig2 {
        match self.bits {
            false => Lane1UlpsSig2::Inactive,
            true => Lane1UlpsSig2::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane1UlpsSig2::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane1UlpsSig2::Active
    }
}
#[doc = "Field `LANE1_ULPS_SIG2` writer - 5:5\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #1 is a data lane. The state of the signal TxUlpsClk is changed if lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane1UlpsSig2W<'a, REG> = crate::BitWriter<'a, REG, Lane1UlpsSig2>;
impl<'a, REG> Lane1UlpsSig2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1UlpsSig2::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane1UlpsSig2::Active)
    }
}
#[doc = "6:6\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #2 is a data lane. The state of the signal TxUlpsClk is changed if lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane2UlpsSig2 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    Active = 1,
}
impl From<Lane2UlpsSig2> for bool {
    #[inline(always)]
    fn from(variant: Lane2UlpsSig2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE2_ULPS_SIG2` reader - 6:6\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #2 is a data lane. The state of the signal TxUlpsClk is changed if lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane2UlpsSig2R = crate::BitReader<Lane2UlpsSig2>;
impl Lane2UlpsSig2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane2UlpsSig2 {
        match self.bits {
            false => Lane2UlpsSig2::Inactive,
            true => Lane2UlpsSig2::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane2UlpsSig2::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane2UlpsSig2::Active
    }
}
#[doc = "Field `LANE2_ULPS_SIG2` writer - 6:6\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #2 is a data lane. The state of the signal TxUlpsClk is changed if lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane2UlpsSig2W<'a, REG> = crate::BitWriter<'a, REG, Lane2UlpsSig2>;
impl<'a, REG> Lane2UlpsSig2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2UlpsSig2::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane2UlpsSig2::Active)
    }
}
#[doc = "Field `RESERVED4` reader - "]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - "]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:7\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #3 is a data lane. The state of the signal TxUlpsClk is changed if lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane3UlpsSig2 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    Active = 1,
}
impl From<Lane3UlpsSig2> for bool {
    #[inline(always)]
    fn from(variant: Lane3UlpsSig2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE3_ULPS_SIG2` reader - 7:7\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #3 is a data lane. The state of the signal TxUlpsClk is changed if lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane3UlpsSig2R = crate::BitReader<Lane3UlpsSig2>;
impl Lane3UlpsSig2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane3UlpsSig2 {
        match self.bits {
            false => Lane3UlpsSig2::Inactive,
            true => Lane3UlpsSig2::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane3UlpsSig2::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane3UlpsSig2::Active
    }
}
#[doc = "Field `LANE3_ULPS_SIG2` writer - 7:7\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #3 is a data lane. The state of the signal TxUlpsClk is changed if lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane3UlpsSig2W<'a, REG> = crate::BitWriter<'a, REG, Lane3UlpsSig2>;
impl<'a, REG> Lane3UlpsSig2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3UlpsSig2::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane3UlpsSig2::Active)
    }
}
#[doc = "8:8\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #4 is a data lane. The state of the signal TxUlpsClk is changed if lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane4UlpsSig2 {
    #[doc = "0: READ: Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    Active = 1,
}
impl From<Lane4UlpsSig2> for bool {
    #[inline(always)]
    fn from(variant: Lane4UlpsSig2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE4_ULPS_SIG2` reader - 8:8\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #4 is a data lane. The state of the signal TxUlpsClk is changed if lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane4UlpsSig2R = crate::BitReader<Lane4UlpsSig2>;
impl Lane4UlpsSig2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane4UlpsSig2 {
        match self.bits {
            false => Lane4UlpsSig2::Inactive,
            true => Lane4UlpsSig2::Active,
        }
    }
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane4UlpsSig2::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane4UlpsSig2::Active
    }
}
#[doc = "Field `LANE4_ULPS_SIG2` writer - 8:8\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #4 is a data lane. The state of the signal TxUlpsClk is changed if lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane4UlpsSig2W<'a, REG> = crate::BitWriter<'a, REG, Lane4UlpsSig2>;
impl<'a, REG> Lane4UlpsSig2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ: Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane4UlpsSig2::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane4UlpsSig2::Active)
    }
}
#[doc = "Field `RESERVED5` reader - "]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RESERVED5` writer - "]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - "]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - "]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "9:9\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #5 is a data lane. The state of the signal TxUlpsClk is changed if lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane5UlpsSig2 {
    #[doc = "0: READ:Inactive state effective. WRITE: Request to change to inactive state"]
    Inactive = 0,
    #[doc = "1: READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    Active = 1,
}
impl From<Lane5UlpsSig2> for bool {
    #[inline(always)]
    fn from(variant: Lane5UlpsSig2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LANE5_ULPS_SIG2` reader - 9:9\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #5 is a data lane. The state of the signal TxUlpsClk is changed if lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane5UlpsSig2R = crate::BitReader<Lane5UlpsSig2>;
impl Lane5UlpsSig2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lane5UlpsSig2 {
        match self.bits {
            false => Lane5UlpsSig2::Inactive,
            true => Lane5UlpsSig2::Active,
        }
    }
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lane5UlpsSig2::Inactive
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Lane5UlpsSig2::Active
    }
}
#[doc = "Field `LANE5_ULPS_SIG2` writer - 9:9\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #5 is a data lane. The state of the signal TxUlpsClk is changed if lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
pub type Lane5UlpsSig2W<'a, REG> = crate::BitWriter<'a, REG, Lane5UlpsSig2>;
impl<'a, REG> Lane5UlpsSig2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READ:Inactive state effective. WRITE: Request to change to inactive state"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lane5UlpsSig2::Inactive)
    }
    #[doc = "READ:Active state effective. WRITE: Change request to active. If the lane is a data lane, TxRequestEsc is asserted and synchronously TxUlpsEsc is asserted for one period of TxClkEsc."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Lane5UlpsSig2::Active)
    }
}
#[doc = "16:16\\]
Indicates when there are still pending operations for VCs configured for HS mode. Forced to 1 when at least one VC is enabled and configured for HS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsBusy {
    #[doc = "0: HS logic is idle"]
    False = 0,
    #[doc = "1: HS logic is active"]
    True = 1,
}
impl From<HsBusy> for bool {
    #[inline(always)]
    fn from(variant: HsBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_BUSY` reader - 16:16\\]
Indicates when there are still pending operations for VCs configured for HS mode. Forced to 1 when at least one VC is enabled and configured for HS mode."]
pub type HsBusyR = crate::BitReader<HsBusy>;
impl HsBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsBusy {
        match self.bits {
            false => HsBusy::False,
            true => HsBusy::True,
        }
    }
    #[doc = "HS logic is idle"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == HsBusy::False
    }
    #[doc = "HS logic is active"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == HsBusy::True
    }
}
#[doc = "Field `HS_BUSY` writer - 16:16\\]
Indicates when there are still pending operations for VCs configured for HS mode. Forced to 1 when at least one VC is enabled and configured for HS mode."]
pub type HsBusyW<'a, REG> = crate::BitWriter<'a, REG, HsBusy>;
impl<'a, REG> HsBusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HS logic is idle"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(HsBusy::False)
    }
    #[doc = "HS logic is active"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(HsBusy::True)
    }
}
#[doc = "17:17\\]
Indicates when there are still pending operations for VCs configured for LP mode. Forced to 1 when at least one VC is enabled and configured for LP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpBusy {
    #[doc = "0: LP logic is idle"]
    False = 0,
    #[doc = "1: LP logic is active"]
    True = 1,
}
impl From<LpBusy> for bool {
    #[inline(always)]
    fn from(variant: LpBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_BUSY` reader - 17:17\\]
Indicates when there are still pending operations for VCs configured for LP mode. Forced to 1 when at least one VC is enabled and configured for LP mode."]
pub type LpBusyR = crate::BitReader<LpBusy>;
impl LpBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpBusy {
        match self.bits {
            false => LpBusy::False,
            true => LpBusy::True,
        }
    }
    #[doc = "LP logic is idle"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == LpBusy::False
    }
    #[doc = "LP logic is active"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == LpBusy::True
    }
}
#[doc = "Field `LP_BUSY` writer - 17:17\\]
Indicates when there are still pending operations for VCs configured for LP mode. Forced to 1 when at least one VC is enabled and configured for LP mode."]
pub type LpBusyW<'a, REG> = crate::BitWriter<'a, REG, LpBusy>;
impl<'a, REG> LpBusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LP logic is idle"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(LpBusy::False)
    }
    #[doc = "LP logic is active"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(LpBusy::True)
    }
}
#[doc = "Field `RESERVED_2` reader - "]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_2` writer - "]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane1_ulps_sig1(&self) -> Lane1UlpsSig1R {
        Lane1UlpsSig1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane2_ulps_sig1(&self) -> Lane2UlpsSig1R {
        Lane2UlpsSig1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane3_ulps_sig1(&self) -> Lane3UlpsSig1R {
        Lane3UlpsSig1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane4_ulps_sig1(&self) -> Lane4UlpsSig1R {
        Lane4UlpsSig1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane5_ulps_sig1(&self) -> Lane5UlpsSig1R {
        Lane5UlpsSig1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #1 is a data lane. The state of the signal TxUlpsClk is changed if lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane1_ulps_sig2(&self) -> Lane1UlpsSig2R {
        Lane1UlpsSig2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #2 is a data lane. The state of the signal TxUlpsClk is changed if lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane2_ulps_sig2(&self) -> Lane2UlpsSig2R {
        Lane2UlpsSig2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #3 is a data lane. The state of the signal TxUlpsClk is changed if lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane3_ulps_sig2(&self) -> Lane3UlpsSig2R {
        Lane3UlpsSig2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #4 is a data lane. The state of the signal TxUlpsClk is changed if lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane4_ulps_sig2(&self) -> Lane4UlpsSig2R {
        Lane4UlpsSig2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #5 is a data lane. The state of the signal TxUlpsClk is changed if lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    pub fn lane5_ulps_sig2(&self) -> Lane5UlpsSig2R {
        Lane5UlpsSig2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates when there are still pending operations for VCs configured for HS mode. Forced to 1 when at least one VC is enabled and configured for HS mode."]
    #[inline(always)]
    pub fn hs_busy(&self) -> HsBusyR {
        HsBusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates when there are still pending operations for VCs configured for LP mode. Forced to 1 when at least one VC is enabled and configured for LP mode."]
    #[inline(always)]
    pub fn lp_busy(&self) -> LpBusyR {
        LpBusyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn reserved_2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane1_ulps_sig1(&mut self) -> Lane1UlpsSig1W<Csi2ComplexioCfg2Spec> {
        Lane1UlpsSig1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane2_ulps_sig1(&mut self) -> Lane2UlpsSig1W<Csi2ComplexioCfg2Spec> {
        Lane2UlpsSig1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2ComplexioCfg2Spec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane3_ulps_sig1(&mut self) -> Lane3UlpsSig1W<Csi2ComplexioCfg2Spec> {
        Lane3UlpsSig1W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2ComplexioCfg2Spec> {
        Reserved2W::new(self, 3)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane4_ulps_sig1(&mut self) -> Lane4UlpsSig1W<Csi2ComplexioCfg2Spec> {
        Lane4UlpsSig1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2ComplexioCfg2Spec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxULPSExit is changed if the lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane5_ulps_sig1(&mut self) -> Lane5UlpsSig1W<Csi2ComplexioCfg2Spec> {
        Lane5UlpsSig1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables the ULPS for the lane #1. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #1 is a data lane. The state of the signal TxUlpsClk is changed if lane #1 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane1_ulps_sig2(&mut self) -> Lane1UlpsSig2W<Csi2ComplexioCfg2Spec> {
        Lane1UlpsSig2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables the ULPS for the lane #2. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #2 is a data lane. The state of the signal TxUlpsClk is changed if lane #2 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane2_ulps_sig2(&mut self) -> Lane2UlpsSig2W<Csi2ComplexioCfg2Spec> {
        Lane2UlpsSig2W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Csi2ComplexioCfg2Spec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables the ULPS for the lane #3. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #3 is a data lane. The state of the signal TxUlpsClk is changed if lane #3 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane3_ulps_sig2(&mut self) -> Lane3UlpsSig2W<Csi2ComplexioCfg2Spec> {
        Lane3UlpsSig2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables the ULPS for the lane #4. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #4 is a data lane. The state of the signal TxUlpsClk is changed if lane #4 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane4_ulps_sig2(&mut self) -> Lane4UlpsSig2W<Csi2ComplexioCfg2Spec> {
        Lane4UlpsSig2W::new(self, 8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Csi2ComplexioCfg2Spec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Csi2ComplexioCfg2Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables the ULPS for the lane #5. The HW shall change the state of the lane to ULPS only when it is in stop state and there is no data pending inside the CSI2 protocol engine and the CSI2 protocol engine has control of the bus (BTA has not been sent). The state of the signal TxRequestEsc is changed if lane #5 is a data lane. The state of the signal TxUlpsClk is changed if lane #5 is a clock lane. There will be a latency depending on the frequency of TxClkExc. This bit should be read back to confirm a write has been effective."]
    #[inline(always)]
    #[must_use]
    pub fn lane5_ulps_sig2(&mut self) -> Lane5UlpsSig2W<Csi2ComplexioCfg2Spec> {
        Lane5UlpsSig2W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates when there are still pending operations for VCs configured for HS mode. Forced to 1 when at least one VC is enabled and configured for HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn hs_busy(&mut self) -> HsBusyW<Csi2ComplexioCfg2Spec> {
        HsBusyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates when there are still pending operations for VCs configured for LP mode. Forced to 1 when at least one VC is enabled and configured for LP mode."]
    #[inline(always)]
    #[must_use]
    pub fn lp_busy(&mut self) -> LpBusyW<Csi2ComplexioCfg2Spec> {
        LpBusyW::new(self, 17)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_2(&mut self) -> Reserved2W<Csi2ComplexioCfg2Spec> {
        Reserved2W::new(self, 18)
    }
}
#[doc = "COMPLEXIO CONFIGURATION REGISTER for the complex IO This register contains the lane configuration for the ULPS for each lane.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ComplexioCfg2Spec;
impl crate::RegisterSpec for Csi2ComplexioCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_complexio_cfg2::R`](R) reader structure"]
impl crate::Readable for Csi2ComplexioCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_complexio_cfg2::W`](W) writer structure"]
impl crate::Writable for Csi2ComplexioCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_COMPLEXIO_CFG2 to value 0"]
impl crate::Resettable for Csi2ComplexioCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
