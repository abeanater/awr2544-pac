#[doc = "Register `STAT_CBUFF_REG2` reader"]
pub type R = crate::R<StatCbuffReg2Spec>;
#[doc = "Register `STAT_CBUFF_REG2` writer"]
pub type W = crate::W<StatCbuffReg2Spec>;
#[doc = "Field `STAT_CBUFF_REG2` reader - 31:0\\]
RESERVED. This does not have coresponding clear or mask"]
pub type StatCbuffReg2R = crate::FieldReader<u32>;
#[doc = "Field `STAT_CBUFF_REG2` writer - 31:0\\]
RESERVED. This does not have coresponding clear or mask"]
pub type StatCbuffReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED. This does not have coresponding clear or mask"]
    #[inline(always)]
    pub fn stat_cbuff_reg2(&self) -> StatCbuffReg2R {
        StatCbuffReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED. This does not have coresponding clear or mask"]
    #[inline(always)]
    #[must_use]
    pub fn stat_cbuff_reg2(&mut self) -> StatCbuffReg2W<StatCbuffReg2Spec> {
        StatCbuffReg2W::new(self, 0)
    }
}
#[doc = "STAT_CBUFF_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCbuffReg2Spec;
impl crate::RegisterSpec for StatCbuffReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_cbuff_reg2::R`](R) reader structure"]
impl crate::Readable for StatCbuffReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`stat_cbuff_reg2::W`](W) writer structure"]
impl crate::Writable for StatCbuffReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_CBUFF_REG2 to value 0"]
impl crate::Resettable for StatCbuffReg2Spec {
    const RESET_VALUE: u32 = 0;
}
