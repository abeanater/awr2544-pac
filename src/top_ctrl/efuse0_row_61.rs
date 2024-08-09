#[doc = "Register `EFUSE0_ROW_61` reader"]
pub type R = crate::R<Efuse0Row61Spec>;
#[doc = "Register `EFUSE0_ROW_61` writer"]
pub type W = crate::W<Efuse0Row61Spec>;
#[doc = "Field `EFUSE0_ROW_61` reader - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse0Row61R = crate::FieldReader<u32>;
#[doc = "Field `EFUSE0_ROW_61` writer - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse0Row61W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    pub fn efuse0_row_61(&self) -> Efuse0Row61R {
        Efuse0Row61R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    #[must_use]
    pub fn efuse0_row_61(&mut self) -> Efuse0Row61W<Efuse0Row61Spec> {
        Efuse0Row61W::new(self, 0)
    }
}
#[doc = "EFUSE0_ROW_61\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse0_row_61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse0_row_61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Efuse0Row61Spec;
impl crate::RegisterSpec for Efuse0Row61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse0_row_61::R`](R) reader structure"]
impl crate::Readable for Efuse0Row61Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse0_row_61::W`](W) writer structure"]
impl crate::Writable for Efuse0Row61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE0_ROW_61 to value 0"]
impl crate::Resettable for Efuse0Row61Spec {
    const RESET_VALUE: u32 = 0;
}
