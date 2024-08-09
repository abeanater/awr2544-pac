#[doc = "Register `RCOSC32K_CTRL` reader"]
pub type R = crate::R<Rcosc32kCtrlSpec>;
#[doc = "Register `RCOSC32K_CTRL` writer"]
pub type W = crate::W<Rcosc32kCtrlSpec>;
#[doc = "Field `stoposc` reader - 2:0\\]
Stop 32KHz RCOSC. Write 3'b111 to stop clock"]
pub type StoposcR = crate::FieldReader;
#[doc = "Field `stoposc` writer - 2:0\\]
Stop 32KHz RCOSC. Write 3'b111 to stop clock"]
pub type StoposcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Stop 32KHz RCOSC. Write 3'b111 to stop clock"]
    #[inline(always)]
    pub fn stoposc(&self) -> StoposcR {
        StoposcR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Stop 32KHz RCOSC. Write 3'b111 to stop clock"]
    #[inline(always)]
    #[must_use]
    pub fn stoposc(&mut self) -> StoposcW<Rcosc32kCtrlSpec> {
        StoposcW::new(self, 0)
    }
}
#[doc = "RCOSC32K_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rcosc32k_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc32k_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcosc32kCtrlSpec;
impl crate::RegisterSpec for Rcosc32kCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcosc32k_ctrl::R`](R) reader structure"]
impl crate::Readable for Rcosc32kCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rcosc32k_ctrl::W`](W) writer structure"]
impl crate::Writable for Rcosc32kCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCOSC32K_CTRL to value 0"]
impl crate::Resettable for Rcosc32kCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
