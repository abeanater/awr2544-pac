#[doc = "Register `EFUSE1_ROW_9` reader"]
pub type R = crate::R<Efuse1Row9Spec>;
#[doc = "Register `EFUSE1_ROW_9` writer"]
pub type W = crate::W<Efuse1Row9Spec>;
#[doc = "Field `EFUSE1_ROW_9` reader - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse1Row9R = crate::FieldReader<u32>;
#[doc = "Field `EFUSE1_ROW_9` writer - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
pub type Efuse1Row9W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    pub fn efuse1_row_9(&self) -> Efuse1Row9R {
        Efuse1Row9R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Captures the EFUSE Value. Refer to EFUSE Mapping XLS for more details"]
    #[inline(always)]
    #[must_use]
    pub fn efuse1_row_9(&mut self) -> Efuse1Row9W<Efuse1Row9Spec> {
        Efuse1Row9W::new(self, 0)
    }
}
#[doc = "EFUSE1_ROW_9\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse1_row_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse1_row_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Efuse1Row9Spec;
impl crate::RegisterSpec for Efuse1Row9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse1_row_9::R`](R) reader structure"]
impl crate::Readable for Efuse1Row9Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse1_row_9::W`](W) writer structure"]
impl crate::Writable for Efuse1Row9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE1_ROW_9 to value 0"]
impl crate::Resettable for Efuse1Row9Spec {
    const RESET_VALUE: u32 = 0;
}
