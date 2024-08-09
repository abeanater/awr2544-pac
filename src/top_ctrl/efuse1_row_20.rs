#[doc = "Register `EFUSE1_ROW_20` reader"]
pub type R = crate::R<Efuse1Row20Spec>;
#[doc = "Register `EFUSE1_ROW_20` writer"]
pub type W = crate::W<Efuse1Row20Spec>;
#[doc = "Field `EFUSE1_ROW_20` reader - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse1Row20R = crate::FieldReader<u32>;
#[doc = "Field `EFUSE1_ROW_20` writer - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse1Row20W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    pub fn efuse1_row_20(&self) -> Efuse1Row20R {
        Efuse1Row20R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    #[must_use]
    pub fn efuse1_row_20(&mut self) -> Efuse1Row20W<Efuse1Row20Spec> {
        Efuse1Row20W::new(self, 0)
    }
}
#[doc = "EFUSE1_ROW_20\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Efuse1Row20Spec;
impl crate::RegisterSpec for Efuse1Row20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse1_row_20::R`](R) reader structure"]
impl crate::Readable for Efuse1Row20Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse1_row_20::W`](W) writer structure"]
impl crate::Writable for Efuse1Row20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE1_ROW_20 to value 0"]
impl crate::Resettable for Efuse1Row20Spec {
    const RESET_VALUE: u32 = 0;
}
