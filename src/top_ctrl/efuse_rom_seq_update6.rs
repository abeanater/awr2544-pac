#[doc = "Register `EFUSE_ROM_SEQ_UPDATE6` reader"]
pub type R = crate::R<EfuseRomSeqUpdate6Spec>;
#[doc = "Register `EFUSE_ROM_SEQ_UPDATE6` writer"]
pub type W = crate::W<EfuseRomSeqUpdate6Spec>;
#[doc = "Field `val` reader - 31:0\\]
EFUSE ROM Seq Update\\[223:192\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
EFUSE ROM Seq Update\\[223:192\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE ROM Seq Update\\[223:192\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE ROM Seq Update\\[223:192\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<EfuseRomSeqUpdate6Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "EFUSE_ROM_SEQ_UPDATE6\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_rom_seq_update6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_rom_seq_update6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseRomSeqUpdate6Spec;
impl crate::RegisterSpec for EfuseRomSeqUpdate6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_rom_seq_update6::R`](R) reader structure"]
impl crate::Readable for EfuseRomSeqUpdate6Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse_rom_seq_update6::W`](W) writer structure"]
impl crate::Writable for EfuseRomSeqUpdate6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_ROM_SEQ_UPDATE6 to value 0"]
impl crate::Resettable for EfuseRomSeqUpdate6Spec {
    const RESET_VALUE: u32 = 0;
}
