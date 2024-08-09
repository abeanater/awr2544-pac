#[doc = "Register `DFT_DMLED_EXEC` reader"]
pub type R = crate::R<DftDmledExecSpec>;
#[doc = "Register `DFT_DMLED_EXEC` writer"]
pub type W = crate::W<DftDmledExecSpec>;
#[doc = "Field `val` reader - 31:0\\]
SW mapping for DMLED Execution Bit 0 : HSM CM4 Execution Bit 1 : RESERVED Bit 2 : MSS CR5 Execution"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
SW mapping for DMLED Execution Bit 0 : HSM CM4 Execution Bit 1 : RESERVED Bit 2 : MSS CR5 Execution"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SW mapping for DMLED Execution Bit 0 : HSM CM4 Execution Bit 1 : RESERVED Bit 2 : MSS CR5 Execution"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SW mapping for DMLED Execution Bit 0 : HSM CM4 Execution Bit 1 : RESERVED Bit 2 : MSS CR5 Execution"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<DftDmledExecSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "DFT_DMLED_EXEC\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_dmled_exec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_dmled_exec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftDmledExecSpec;
impl crate::RegisterSpec for DftDmledExecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_dmled_exec::R`](R) reader structure"]
impl crate::Readable for DftDmledExecSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_dmled_exec::W`](W) writer structure"]
impl crate::Writable for DftDmledExecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFT_DMLED_EXEC to value 0"]
impl crate::Resettable for DftDmledExecSpec {
    const RESET_VALUE: u32 = 0;
}
