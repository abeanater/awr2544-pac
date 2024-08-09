#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE_ECC` reader - 0:0\\]
Enable ECC"]
pub type EnableEccR = crate::BitReader;
#[doc = "Field `ENABLE_ECC` writer - 0:0\\]
Enable ECC"]
pub type EnableEccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_ECC_CHECK` reader - 1:1\\]
Enable ECC check"]
pub type EnableEccCheckR = crate::BitReader;
#[doc = "Field `ENABLE_ECC_CHECK` writer - 1:1\\]
Enable ECC check"]
pub type EnableEccCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_RMW` reader - 2:2\\]
Enable rmw"]
pub type EnableRmwR = crate::BitReader;
#[doc = "Field `ENABLE_RMW` writer - 2:2\\]
Enable rmw"]
pub type EnableRmwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SINGLE_BIT` reader - 3:3\\]
Force Single Bit Error"]
pub type ForceSingleBitR = crate::BitReader;
#[doc = "Field `FORCE_SINGLE_BIT` writer - 3:3\\]
Force Single Bit Error"]
pub type ForceSingleBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DOUBLE_BIT` reader - 4:4\\]
Force Double Bit Error"]
pub type ForceDoubleBitR = crate::BitReader;
#[doc = "Field `FORCE_DOUBLE_BIT` writer - 4:4\\]
Force Double Bit Error"]
pub type ForceDoubleBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERROR_ON` reader - 5:5\\]
Force Error on any RAM read"]
pub type ForceErrorOnR = crate::BitReader;
#[doc = "Field `FORCE_ERROR_ON` writer - 5:5\\]
Force Error on any RAM read"]
pub type ForceErrorOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERROR_ONLY` reader - 6:6\\]
Force Error only once"]
pub type ForceErrorOnlyR = crate::BitReader;
#[doc = "Field `FORCE_ERROR_ONLY` writer - 6:6\\]
Force Error only once"]
pub type ForceErrorOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_FOR_PARITY` reader - 7:7\\]
check for parity errors"]
pub type CheckForParityR = crate::BitReader;
#[doc = "Field `CHECK_FOR_PARITY` writer - 7:7\\]
check for parity errors"]
pub type CheckForParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_FOR_SVBUS` reader - 8:8\\]
check for svbus timeout errors"]
pub type CheckForSvbusR = crate::BitReader;
#[doc = "Field `CHECK_FOR_SVBUS` writer - 8:8\\]
check for svbus timeout errors"]
pub type CheckForSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable ECC"]
    #[inline(always)]
    pub fn enable_ecc(&self) -> EnableEccR {
        EnableEccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable ECC check"]
    #[inline(always)]
    pub fn enable_ecc_check(&self) -> EnableEccCheckR {
        EnableEccCheckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable rmw"]
    #[inline(always)]
    pub fn enable_rmw(&self) -> EnableRmwR {
        EnableRmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Single Bit Error"]
    #[inline(always)]
    pub fn force_single_bit(&self) -> ForceSingleBitR {
        ForceSingleBitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Double Bit Error"]
    #[inline(always)]
    pub fn force_double_bit(&self) -> ForceDoubleBitR {
        ForceDoubleBitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Error on any RAM read"]
    #[inline(always)]
    pub fn force_error_on(&self) -> ForceErrorOnR {
        ForceErrorOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Error only once"]
    #[inline(always)]
    pub fn force_error_only(&self) -> ForceErrorOnlyR {
        ForceErrorOnlyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
check for parity errors"]
    #[inline(always)]
    pub fn check_for_parity(&self) -> CheckForParityR {
        CheckForParityR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
check for svbus timeout errors"]
    #[inline(always)]
    pub fn check_for_svbus(&self) -> CheckForSvbusR {
        CheckForSvbusR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable ECC"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ecc(&mut self) -> EnableEccW<CtrlSpec> {
        EnableEccW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable ECC check"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ecc_check(&mut self) -> EnableEccCheckW<CtrlSpec> {
        EnableEccCheckW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable rmw"]
    #[inline(always)]
    #[must_use]
    pub fn enable_rmw(&mut self) -> EnableRmwW<CtrlSpec> {
        EnableRmwW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Single Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn force_single_bit(&mut self) -> ForceSingleBitW<CtrlSpec> {
        ForceSingleBitW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Double Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn force_double_bit(&mut self) -> ForceDoubleBitW<CtrlSpec> {
        ForceDoubleBitW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Error on any RAM read"]
    #[inline(always)]
    #[must_use]
    pub fn force_error_on(&mut self) -> ForceErrorOnW<CtrlSpec> {
        ForceErrorOnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Error only once"]
    #[inline(always)]
    #[must_use]
    pub fn force_error_only(&mut self) -> ForceErrorOnlyW<CtrlSpec> {
        ForceErrorOnlyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
check for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn check_for_parity(&mut self) -> CheckForParityW<CtrlSpec> {
        CheckForParityW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
check for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn check_for_svbus(&mut self) -> CheckForSvbusW<CtrlSpec> {
        CheckForSvbusW::new(self, 8)
    }
}
#[doc = "ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
