#[doc = "Register `TGINTFLAG` reader"]
pub type R = crate::R<TgintflagSpec>;
#[doc = "Register `TGINTFLAG` writer"]
pub type W = crate::W<TgintflagSpec>;
#[doc = "Field `INTFLGSUS` reader - 15:0\\]
Transfer group interrupt flag for ΓÇ£transfer suspendΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer suspendedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENSUSx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGSUSx is set right after the transfer from transfer group x is suspended. 0 =No ΓÇ¥transfer suspendedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGSUSx. Note: Read Clear Behavior Reading the interrupt vector registers TGINTVECT0 or TGINTVECT1 automatically clears the interrupt flag bit INTFLGRDYx referenced by the vector number given by INTVECT0/INTVECT1 bits, if SUSPEND0/SUPEND1 bit in the Vector registers is ΓÇÿ0ΓÇÖ."]
pub type IntflgsusR = crate::FieldReader<u16>;
#[doc = "Field `INTFLGSUS` writer - 15:0\\]
Transfer group interrupt flag for ΓÇ£transfer suspendΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer suspendedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENSUSx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGSUSx is set right after the transfer from transfer group x is suspended. 0 =No ΓÇ¥transfer suspendedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGSUSx. Note: Read Clear Behavior Reading the interrupt vector registers TGINTVECT0 or TGINTVECT1 automatically clears the interrupt flag bit INTFLGRDYx referenced by the vector number given by INTVECT0/INTVECT1 bits, if SUSPEND0/SUPEND1 bit in the Vector registers is ΓÇÿ0ΓÇÖ."]
pub type IntflgsusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INTFLGRDY` reader - 31:16\\]
Transfer group interrupt flag for ΓÇ£transfer finishedΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer finishedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENRDYx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGRDYx is set right after the transfer from transfer group x is finished. 0 =No ΓÇ£transfer finishedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGRDYx. Write: 1 = Clears the corresponding bit flag. 0 = Has no effect."]
pub type IntflgrdyR = crate::FieldReader<u16>;
#[doc = "Field `INTFLGRDY` writer - 31:16\\]
Transfer group interrupt flag for ΓÇ£transfer finishedΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer finishedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENRDYx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGRDYx is set right after the transfer from transfer group x is finished. 0 =No ΓÇ£transfer finishedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGRDYx. Write: 1 = Clears the corresponding bit flag. 0 = Has no effect."]
pub type IntflgrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt flag for ΓÇ£transfer suspendΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer suspendedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENSUSx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGSUSx is set right after the transfer from transfer group x is suspended. 0 =No ΓÇ¥transfer suspendedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGSUSx. Note: Read Clear Behavior Reading the interrupt vector registers TGINTVECT0 or TGINTVECT1 automatically clears the interrupt flag bit INTFLGRDYx referenced by the vector number given by INTVECT0/INTVECT1 bits, if SUSPEND0/SUPEND1 bit in the Vector registers is ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn intflgsus(&self) -> IntflgsusR {
        IntflgsusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt flag for ΓÇ£transfer finishedΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer finishedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENRDYx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGRDYx is set right after the transfer from transfer group x is finished. 0 =No ΓÇ£transfer finishedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGRDYx. Write: 1 = Clears the corresponding bit flag. 0 = Has no effect."]
    #[inline(always)]
    pub fn intflgrdy(&self) -> IntflgrdyR {
        IntflgrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt flag for ΓÇ£transfer suspendΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer suspendedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENSUSx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGSUSx is set right after the transfer from transfer group x is suspended. 0 =No ΓÇ¥transfer suspendedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGSUSx. Note: Read Clear Behavior Reading the interrupt vector registers TGINTVECT0 or TGINTVECT1 automatically clears the interrupt flag bit INTFLGRDYx referenced by the vector number given by INTVECT0/INTVECT1 bits, if SUSPEND0/SUPEND1 bit in the Vector registers is ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn intflgsus(&mut self) -> IntflgsusW<TgintflagSpec> {
        IntflgsusW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt flag for ΓÇ£transfer finishedΓÇ¥ interrupt. Read: 1 =A ΓÇ£transfer finishedΓÇ¥ interrupt from transfer group x occurred. No matter whether the interrupt is enabled or disabled (INTENRDYx = donΓÇÖt care) or whether the interrupt is mapped to line INT0 or INT1, INTFLGRDYx is set right after the transfer from transfer group x is finished. 0 =No ΓÇ£transfer finishedΓÇ¥ interrupt occurred since last clearing of the flag INTFLGRDYx. Write: 1 = Clears the corresponding bit flag. 0 = Has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn intflgrdy(&mut self) -> IntflgrdyW<TgintflagSpec> {
        IntflgrdyW::new(self, 16)
    }
}
#[doc = "Transfer Group Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgintflagSpec;
impl crate::RegisterSpec for TgintflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgintflag::R`](R) reader structure"]
impl crate::Readable for TgintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`tgintflag::W`](W) writer structure"]
impl crate::Writable for TgintflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGINTFLAG to value 0"]
impl crate::Resettable for TgintflagSpec {
    const RESET_VALUE: u32 = 0;
}
