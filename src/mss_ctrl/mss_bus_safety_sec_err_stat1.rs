#[doc = "Register `MSS_BUS_SAFETY_SEC_ERR_STAT1` reader"]
pub type R = crate::R<MssBusSafetySecErrStat1Spec>;
#[doc = "Register `MSS_BUS_SAFETY_SEC_ERR_STAT1` writer"]
pub type W = crate::W<MssBusSafetySecErrStat1Spec>;
#[doc = "Field `mss2r5ss` reader - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Mss2r5ssR = crate::BitReader;
#[doc = "Field `mss2r5ss` writer - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Mss2r5ssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r5ss2mss` reader - 1:1\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type R5ss2mssR = crate::BitReader;
#[doc = "Field `r5ss2mss` writer - 1:1\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type R5ss2mssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss2r5ss` reader - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Dss2r5ssR = crate::BitReader;
#[doc = "Field `dss2r5ss` writer - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type Dss2r5ssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r5ss2dss` reader - 3:3\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type R5ss2dssR = crate::BitReader;
#[doc = "Field `r5ss2dss` writer - 3:3\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type R5ss2dssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l2ram2` reader - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram2R = crate::BitReader;
#[doc = "Field `l2ram2` writer - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
pub type L2ram2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn mss2r5ss(&self) -> Mss2r5ssR {
        Mss2r5ssR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn r5ss2mss(&self) -> R5ss2mssR {
        R5ss2mssR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn dss2r5ss(&self) -> Dss2r5ssR {
        Dss2r5ssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn r5ss2dss(&self) -> R5ss2dssR {
        R5ss2dssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    pub fn l2ram2(&self) -> L2ram2R {
        L2ram2R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn mss2r5ss(&mut self) -> Mss2r5ssW<MssBusSafetySecErrStat1Spec> {
        Mss2r5ssW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn r5ss2mss(&mut self) -> R5ss2mssW<MssBusSafetySecErrStat1Spec> {
        R5ss2mssW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn dss2r5ss(&mut self) -> Dss2r5ssW<MssBusSafetySecErrStat1Spec> {
        Dss2r5ssW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn r5ss2dss(&mut self) -> R5ss2dssW<MssBusSafetySecErrStat1Spec> {
        R5ss2dssW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Bus safety single-bit-error of Node mentioned in the field"]
    #[inline(always)]
    #[must_use]
    pub fn l2ram2(&mut self) -> L2ram2W<MssBusSafetySecErrStat1Spec> {
        L2ram2W::new(self, 4)
    }
}
#[doc = "MSS_BUS_SAFETY_SEC_ERR_STAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_bus_safety_sec_err_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_bus_safety_sec_err_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBusSafetySecErrStat1Spec;
impl crate::RegisterSpec for MssBusSafetySecErrStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_bus_safety_sec_err_stat1::R`](R) reader structure"]
impl crate::Readable for MssBusSafetySecErrStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_bus_safety_sec_err_stat1::W`](W) writer structure"]
impl crate::Writable for MssBusSafetySecErrStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BUS_SAFETY_SEC_ERR_STAT1 to value 0"]
impl crate::Resettable for MssBusSafetySecErrStat1Spec {
    const RESET_VALUE: u32 = 0;
}
