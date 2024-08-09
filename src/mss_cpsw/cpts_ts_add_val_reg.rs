#[doc = "Register `CPTS_TS_ADD_VAL_REG` reader"]
pub type R = crate::R<CptsTsAddValRegSpec>;
#[doc = "Register `CPTS_TS_ADD_VAL_REG` writer"]
pub type W = crate::W<CptsTsAddValRegSpec>;
#[doc = "Field `ADD_VALUE` reader - 2:0\\]
Add Value"]
pub type AddValueR = crate::FieldReader;
#[doc = "Field `ADD_VALUE` writer - 2:0\\]
Add Value"]
pub type AddValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Add Value"]
    #[inline(always)]
    pub fn add_value(&self) -> AddValueR {
        AddValueR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Add Value"]
    #[inline(always)]
    #[must_use]
    pub fn add_value(&mut self) -> AddValueW<CptsTsAddValRegSpec> {
        AddValueW::new(self, 0)
    }
}
#[doc = "TS Add Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_add_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_add_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsAddValRegSpec;
impl crate::RegisterSpec for CptsTsAddValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_add_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsAddValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_add_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsAddValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_ADD_VAL_REG to value 0"]
impl crate::Resettable for CptsTsAddValRegSpec {
    const RESET_VALUE: u32 = 0;
}
