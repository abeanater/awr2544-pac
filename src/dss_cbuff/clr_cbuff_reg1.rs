#[doc = "Register `CLR_CBUFF_REG1` reader"]
pub type R = crate::R<ClrCbuffReg1Spec>;
#[doc = "Register `CLR_CBUFF_REG1` writer"]
pub type W = crate::W<ClrCbuffReg1Spec>;
#[doc = "Field `CLR_CBUFF_REG1` reader - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG1. Write 0x1 to Clear the field"]
pub type ClrCbuffReg1R = crate::FieldReader<u32>;
#[doc = "Field `CLR_CBUFF_REG1` writer - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG1. Write 0x1 to Clear the field"]
pub type ClrCbuffReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG1. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn clr_cbuff_reg1(&self) -> ClrCbuffReg1R {
        ClrCbuffReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG1. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn clr_cbuff_reg1(&mut self) -> ClrCbuffReg1W<ClrCbuffReg1Spec> {
        ClrCbuffReg1W::new(self, 0)
    }
}
#[doc = "CLR_CBUFF_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrCbuffReg1Spec;
impl crate::RegisterSpec for ClrCbuffReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_cbuff_reg1::R`](R) reader structure"]
impl crate::Readable for ClrCbuffReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`clr_cbuff_reg1::W`](W) writer structure"]
impl crate::Writable for ClrCbuffReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_CBUFF_REG1 to value 0"]
impl crate::Resettable for ClrCbuffReg1Spec {
    const RESET_VALUE: u32 = 0;
}
