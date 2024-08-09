#[doc = "Register `CCC_DCC_COMMON` reader"]
pub type R = crate::R<CccDccCommonSpec>;
#[doc = "Register `CCC_DCC_COMMON` writer"]
pub type W = crate::W<CccDccCommonSpec>;
#[doc = "Field `enable_cccb_err_rstn` reader - 8:8\\]
1'b0: Enable CCCB error to generate WD restn. 1'b1: disables CCCB error to generate WD restn."]
pub type EnableCccbErrRstnR = crate::BitReader;
#[doc = "Field `enable_cccb_err_rstn` writer - 8:8\\]
1'b0: Enable CCCB error to generate WD restn. 1'b1: disables CCCB error to generate WD restn."]
pub type EnableCccbErrRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable_cccb_err_nmi` reader - 12:12\\]
1'b0:Enable CCCB error to generate NMI. 1'b1:disables CCCB error to generate NMI."]
pub type EnableCccbErrNmiR = crate::BitReader;
#[doc = "Field `enable_cccb_err_nmi` writer - 12:12\\]
1'b0:Enable CCCB error to generate NMI. 1'b1:disables CCCB error to generate NMI."]
pub type EnableCccbErrNmiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
1'b0: Enable CCCB error to generate WD restn. 1'b1: disables CCCB error to generate WD restn."]
    #[inline(always)]
    pub fn enable_cccb_err_rstn(&self) -> EnableCccbErrRstnR {
        EnableCccbErrRstnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
1'b0:Enable CCCB error to generate NMI. 1'b1:disables CCCB error to generate NMI."]
    #[inline(always)]
    pub fn enable_cccb_err_nmi(&self) -> EnableCccbErrNmiR {
        EnableCccbErrNmiR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
1'b0: Enable CCCB error to generate WD restn. 1'b1: disables CCCB error to generate WD restn."]
    #[inline(always)]
    #[must_use]
    pub fn enable_cccb_err_rstn(&mut self) -> EnableCccbErrRstnW<CccDccCommonSpec> {
        EnableCccbErrRstnW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
1'b0:Enable CCCB error to generate NMI. 1'b1:disables CCCB error to generate NMI."]
    #[inline(always)]
    #[must_use]
    pub fn enable_cccb_err_nmi(&mut self) -> EnableCccbErrNmiW<CccDccCommonSpec> {
        EnableCccbErrNmiW::new(self, 12)
    }
}
#[doc = "CCC_DCC_COMMON\n\nYou can [`read`](crate::Reg::read) this register and get [`ccc_dcc_common::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccc_dcc_common::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccDccCommonSpec;
impl crate::RegisterSpec for CccDccCommonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccc_dcc_common::R`](R) reader structure"]
impl crate::Readable for CccDccCommonSpec {}
#[doc = "`write(|w| ..)` method takes [`ccc_dcc_common::W`](W) writer structure"]
impl crate::Writable for CccDccCommonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCC_DCC_COMMON to value 0"]
impl crate::Resettable for CccDccCommonSpec {
    const RESET_VALUE: u32 = 0;
}
