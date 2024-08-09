#[doc = "Register `R5B_AHB_BASE` reader"]
pub type R = crate::R<R5bAhbBaseSpec>;
#[doc = "Register `R5B_AHB_BASE` writer"]
pub type W = crate::W<R5bAhbBaseSpec>;
#[doc = "Field `ahb_base` reader - 19:0\\]
Ti internal Register. Modifying this register is not recommended Decides the base address of ahb region"]
pub type AhbBaseR = crate::FieldReader<u32>;
#[doc = "Field `ahb_base` writer - 19:0\\]
Ti internal Register. Modifying this register is not recommended Decides the base address of ahb region"]
pub type AhbBaseW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Ti internal Register. Modifying this register is not recommended Decides the base address of ahb region"]
    #[inline(always)]
    pub fn ahb_base(&self) -> AhbBaseR {
        AhbBaseR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Ti internal Register. Modifying this register is not recommended Decides the base address of ahb region"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_base(&mut self) -> AhbBaseW<R5bAhbBaseSpec> {
        AhbBaseW::new(self, 0)
    }
}
#[doc = "R5B_AHB_BASE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5b_ahb_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5b_ahb_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5bAhbBaseSpec;
impl crate::RegisterSpec for R5bAhbBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5b_ahb_base::R`](R) reader structure"]
impl crate::Readable for R5bAhbBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`r5b_ahb_base::W`](W) writer structure"]
impl crate::Writable for R5bAhbBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5B_AHB_BASE to value 0"]
impl crate::Resettable for R5bAhbBaseSpec {
    const RESET_VALUE: u32 = 0;
}
