#[doc = "Register `LIMP_MODE_EN` reader"]
pub type R = crate::R<LimpModeEnSpec>;
#[doc = "Register `LIMP_MODE_EN` writer"]
pub type W = crate::W<LimpModeEnSpec>;
#[doc = "Field `dcca_en` reader - 2:0\\]
Enable MSS_DCCA Error to generate Limp mode 3'b000: MSS_DCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_DCCA Error will generate Limp mode (multibit 111)"]
pub type DccaEnR = crate::FieldReader;
#[doc = "Field `dcca_en` writer - 2:0\\]
Enable MSS_DCCA Error to generate Limp mode 3'b000: MSS_DCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_DCCA Error will generate Limp mode (multibit 111)"]
pub type DccaEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ccca_en` reader - 6:4\\]
Enable MSS_CCCA Error to generate Limp mode 3'b000: MSS_CCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_CCCA Error will generate Limp mode (multibit 111)"]
pub type CccaEnR = crate::FieldReader;
#[doc = "Field `ccca_en` writer - 6:4\\]
Enable MSS_CCCA Error to generate Limp mode 3'b000: MSS_CCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_CCCA Error will generate Limp mode (multibit 111)"]
pub type CccaEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `force_rcclk_en` reader - 10:8\\]
Force the RCCLK on when limp mode is detected 3'b000: The RCCLK will not be forced on when limp mode is detected (multibit 000) 3'b111 : The RCCLK will be forced on when limp mode is detected (multibit 111)"]
pub type ForceRcclkEnR = crate::FieldReader;
#[doc = "Field `force_rcclk_en` writer - 10:8\\]
Force the RCCLK on when limp mode is detected 3'b000: The RCCLK will not be forced on when limp mode is detected (multibit 000) 3'b111 : The RCCLK will be forced on when limp mode is detected (multibit 111)"]
pub type ForceRcclkEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable MSS_DCCA Error to generate Limp mode 3'b000: MSS_DCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_DCCA Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    pub fn dcca_en(&self) -> DccaEnR {
        DccaEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable MSS_CCCA Error to generate Limp mode 3'b000: MSS_CCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_CCCA Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    pub fn ccca_en(&self) -> CccaEnR {
        CccaEnR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Force the RCCLK on when limp mode is detected 3'b000: The RCCLK will not be forced on when limp mode is detected (multibit 000) 3'b111 : The RCCLK will be forced on when limp mode is detected (multibit 111)"]
    #[inline(always)]
    pub fn force_rcclk_en(&self) -> ForceRcclkEnR {
        ForceRcclkEnR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable MSS_DCCA Error to generate Limp mode 3'b000: MSS_DCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_DCCA Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn dcca_en(&mut self) -> DccaEnW<LimpModeEnSpec> {
        DccaEnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Enable MSS_CCCA Error to generate Limp mode 3'b000: MSS_CCCA Error will not generate Limp mode (multibit 000) 3'b111 : MSS_CCCA Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_en(&mut self) -> CccaEnW<LimpModeEnSpec> {
        CccaEnW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Force the RCCLK on when limp mode is detected 3'b000: The RCCLK will not be forced on when limp mode is detected (multibit 000) 3'b111 : The RCCLK will be forced on when limp mode is detected (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn force_rcclk_en(&mut self) -> ForceRcclkEnW<LimpModeEnSpec> {
        ForceRcclkEnW::new(self, 8)
    }
}
#[doc = "LIMP_MODE_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`limp_mode_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limp_mode_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimpModeEnSpec;
impl crate::RegisterSpec for LimpModeEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limp_mode_en::R`](R) reader structure"]
impl crate::Readable for LimpModeEnSpec {}
#[doc = "`write(|w| ..)` method takes [`limp_mode_en::W`](W) writer structure"]
impl crate::Writable for LimpModeEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMP_MODE_EN to value 0"]
impl crate::Resettable for LimpModeEnSpec {
    const RESET_VALUE: u32 = 0;
}
