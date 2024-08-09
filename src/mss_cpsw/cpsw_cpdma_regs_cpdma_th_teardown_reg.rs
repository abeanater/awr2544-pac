#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_TEARDOWN_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaThTeardownRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_TEARDOWN_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaThTeardownRegSpec>;
#[doc = "Field `CPDMA_THOST_TEARDOWN` reader - 2:0\\]
CPDMA THost Teardown Channel"]
pub type CpdmaThostTeardownR = crate::FieldReader;
#[doc = "Field `CPDMA_THOST_TEARDOWN` writer - 2:0\\]
CPDMA THost Teardown Channel"]
pub type CpdmaThostTeardownW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPDMA_THOST_TEARDOWN` reader - 31:31\\]
CPDMA THost Teardown Ready"]
pub type CpdmaThostTeardownR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_TEARDOWN` writer - 31:31\\]
CPDMA THost Teardown Ready"]
pub type CpdmaThostTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
CPDMA THost Teardown Channel"]
    #[inline(always)]
    pub fn cpdma_thost_teardown(&self) -> CpdmaThostTeardownR {
        CpdmaThostTeardownR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA THost Teardown Ready"]
    #[inline(always)]
    pub fn cpdma_thost_teardown(&self) -> CpdmaThostTeardownR {
        CpdmaThostTeardownR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
CPDMA THost Teardown Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_teardown(
        &mut self,
    ) -> CpdmaThostTeardownW<CpswCpdmaRegsCpdmaThTeardownRegSpec> {
        CpdmaThostTeardownW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
CPDMA THost Teardown Ready"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_teardown(
        &mut self,
    ) -> CpdmaThostTeardownW<CpswCpdmaRegsCpdmaThTeardownRegSpec> {
        CpdmaThostTeardownW::new(self, 31)
    }
}
#[doc = "CPDMA THost Teardown Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_teardown_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_teardown_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaThTeardownRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaThTeardownRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_th_teardown_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaThTeardownRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_th_teardown_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaThTeardownRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_TH_TEARDOWN_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaThTeardownRegSpec {
    const RESET_VALUE: u32 = 0;
}
