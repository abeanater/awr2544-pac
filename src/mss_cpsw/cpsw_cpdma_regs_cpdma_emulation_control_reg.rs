#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_EMULATION_CONTROL_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaEmulationControlRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_EMULATION_CONTROL_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaEmulationControlRegSpec>;
#[doc = "Field `CPDMA_THOST_BUFFER_1` reader - 0:0\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBuffer1R = crate::BitReader;
#[doc = "Field `CPDMA_THOST_BUFFER_1` writer - 0:0\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBuffer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_BUFFER` reader - 1:1\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBufferR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_BUFFER` writer - 1:1\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBufferW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    pub fn cpdma_thost_buffer_1(&self) -> CpdmaThostBuffer1R {
        CpdmaThostBuffer1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    pub fn cpdma_thost_buffer(&self) -> CpdmaThostBufferR {
        CpdmaThostBufferR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_buffer_1(
        &mut self,
    ) -> CpdmaThostBuffer1W<CpswCpdmaRegsCpdmaEmulationControlRegSpec> {
        CpdmaThostBuffer1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_buffer(
        &mut self,
    ) -> CpdmaThostBufferW<CpswCpdmaRegsCpdmaEmulationControlRegSpec> {
        CpdmaThostBufferW::new(self, 1)
    }
}
#[doc = "CPDMA THost Buffer Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_emulation_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_emulation_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaEmulationControlRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaEmulationControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_emulation_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaEmulationControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_emulation_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaEmulationControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_EMULATION_CONTROL_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaEmulationControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
