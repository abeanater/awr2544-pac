#[doc = "Register `PARAMADDR` reader"]
pub type R = crate::R<ParamaddrSpec>;
#[doc = "Register `PARAMADDR` writer"]
pub type W = crate::W<ParamaddrSpec>;
#[doc = "Field `paramaddr` reader - 5:0\\]
Index of the current parameter set being executed from PARAM RAM ."]
pub type ParamaddrR = crate::FieldReader;
#[doc = "Field `paramaddr` writer - 5:0\\]
Index of the current parameter set being executed from PARAM RAM ."]
pub type ParamaddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Index of the current parameter set being executed from PARAM RAM ."]
    #[inline(always)]
    pub fn paramaddr(&self) -> ParamaddrR {
        ParamaddrR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Index of the current parameter set being executed from PARAM RAM ."]
    #[inline(always)]
    #[must_use]
    pub fn paramaddr(&mut self) -> ParamaddrW<ParamaddrSpec> {
        ParamaddrW::new(self, 0)
    }
}
#[doc = "PARAMADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`paramaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paramaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamaddrSpec;
impl crate::RegisterSpec for ParamaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paramaddr::R`](R) reader structure"]
impl crate::Readable for ParamaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`paramaddr::W`](W) writer structure"]
impl crate::Writable for ParamaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAMADDR to value 0"]
impl crate::Resettable for ParamaddrSpec {
    const RESET_VALUE: u32 = 0;
}
