#[doc = "Register `MSS_BUS_SAFETY_SEC_ERR_STAT0` reader"]
pub type R = crate::R<MssBusSafetySecErrStat0Spec>;
#[doc = "Register `MSS_BUS_SAFETY_SEC_ERR_STAT0` writer"]
pub type W = crate::W<MssBusSafetySecErrStat0Spec>;
#[doc = "Field `cr5a_rd` reader - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aRdR = crate::BitReader;
#[doc = "Field `cr5a_rd` writer - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr5a_wr` reader - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aWrR = crate::BitReader;
#[doc = "Field `cr5a_wr` writer - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr5a_slv` reader - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aSlvR = crate::BitReader;
#[doc = "Field `cr5a_slv` writer - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Cr5aSlvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l2ram0` reader - 25:25\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram0R = crate::BitReader;
#[doc = "Field `l2ram0` writer - 25:25\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l2ram1` reader - 26:26\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram1R = crate::BitReader;
#[doc = "Field `l2ram1` writer - 26:26\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn cr5a_rd(&self) -> Cr5aRdR {
        Cr5aRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn cr5a_wr(&self) -> Cr5aWrR {
        Cr5aWrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn cr5a_slv(&self) -> Cr5aSlvR {
        Cr5aSlvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn l2ram0(&self) -> L2ram0R {
        L2ram0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn l2ram1(&self) -> L2ram1R {
        L2ram1R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn cr5a_rd(&mut self) -> Cr5aRdW<MssBusSafetySecErrStat0Spec> {
        Cr5aRdW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn cr5a_wr(&mut self) -> Cr5aWrW<MssBusSafetySecErrStat0Spec> {
        Cr5aWrW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn cr5a_slv(&mut self) -> Cr5aSlvW<MssBusSafetySecErrStat0Spec> {
        Cr5aSlvW::new(self, 4)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn l2ram0(&mut self) -> L2ram0W<MssBusSafetySecErrStat0Spec> {
        L2ram0W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn l2ram1(&mut self) -> L2ram1W<MssBusSafetySecErrStat0Spec> {
        L2ram1W::new(self, 26)
    }
}
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_sec_err_stat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_sec_err_stat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBusSafetySecErrStat0Spec;
impl crate::RegisterSpec for MssBusSafetySecErrStat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_bus_safety_sec_err_stat0::R`](R) reader structure"]
impl crate::Readable for MssBusSafetySecErrStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_bus_safety_sec_err_stat0::W`](W) writer structure"]
impl crate::Writable for MssBusSafetySecErrStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BUS_SAFETY_SEC_ERR_STAT0 to value 0"]
impl crate::Resettable for MssBusSafetySecErrStat0Spec {
    const RESET_VALUE: u32 = 0;
}
