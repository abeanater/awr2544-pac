#[doc = "Register `DFT_DMLED_STATUS` reader"]
pub type R = crate::R<DftDmledStatusSpec>;
#[doc = "Register `DFT_DMLED_STATUS` writer"]
pub type W = crate::W<DftDmledStatusSpec>;
#[doc = "Field `val` reader - 31:0\\]
SW mapping for DMLED Status Bit 0 : HSM CM4 Status Bit 1 : RESERVED Bit 2 : MSS CR5 Status"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
SW mapping for DMLED Status Bit 0 : HSM CM4 Status Bit 1 : RESERVED Bit 2 : MSS CR5 Status"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SW mapping for DMLED Status Bit 0 : HSM CM4 Status Bit 1 : RESERVED Bit 2 : MSS CR5 Status"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SW mapping for DMLED Status Bit 0 : HSM CM4 Status Bit 1 : RESERVED Bit 2 : MSS CR5 Status"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<DftDmledStatusSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "DFT_DMLED_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_dmled_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_dmled_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftDmledStatusSpec;
impl crate::RegisterSpec for DftDmledStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_dmled_status::R`](R) reader structure"]
impl crate::Readable for DftDmledStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_dmled_status::W`](W) writer structure"]
impl crate::Writable for DftDmledStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFT_DMLED_STATUS to value 0"]
impl crate::Resettable for DftDmledStatusSpec {
    const RESET_VALUE: u32 = 0;
}
