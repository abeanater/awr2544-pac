#[doc = "Register `INTPRIORITY_91` reader"]
pub type R = crate::R<Intpriority91Spec>;
#[doc = "Register `INTPRIORITY_91` writer"]
pub type W = crate::W<Intpriority91Spec>;
#[doc = "Field `PRI` reader - 3:0\\]
This is the priority for interrupt Q. If two interrupts have the same priority, then whichever interrupt has the lower number Q wins arbitration 0 Highest Priority 15 Lowest Priority (Default)"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - 3:0\\]
This is the priority for interrupt Q. If two interrupts have the same priority, then whichever interrupt has the lower number Q wins arbitration 0 Highest Priority 15 Lowest Priority (Default)"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES19` reader - 31:4\\]
RESERVE FIELD"]
pub type Res19R = crate::FieldReader<u32>;
#[doc = "Field `RES19` writer - 31:4\\]
RESERVE FIELD"]
pub type Res19W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This is the priority for interrupt Q. If two interrupts have the same priority, then whichever interrupt has the lower number Q wins arbitration 0 Highest Priority 15 Lowest Priority (Default)"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res19(&self) -> Res19R {
        Res19R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This is the priority for interrupt Q. If two interrupts have the same priority, then whichever interrupt has the lower number Q wins arbitration 0 Highest Priority 15 Lowest Priority (Default)"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<Intpriority91Spec> {
        PriW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res19(&mut self) -> Res19W<Intpriority91Spec> {
        Res19W::new(self, 4)
    }
}
#[doc = "Interrupt Q Priority Register (Q is 0 to 255 , Q= M+1 x 32) h1000 + Q x h95\n\nYou can [`read`](crate::Reg::read) this register and get [`intpriority_91::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpriority_91::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intpriority91Spec;
impl crate::RegisterSpec for Intpriority91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpriority_91::R`](R) reader structure"]
impl crate::Readable for Intpriority91Spec {}
#[doc = "`write(|w| ..)` method takes [`intpriority_91::W`](W) writer structure"]
impl crate::Writable for Intpriority91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPRIORITY_91 to value 0"]
impl crate::Resettable for Intpriority91Spec {
    const RESET_VALUE: u32 = 0;
}
