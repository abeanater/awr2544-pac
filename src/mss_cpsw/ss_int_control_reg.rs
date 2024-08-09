#[doc = "Register `SS_INT_CONTROL_REG` reader"]
pub type R = crate::R<SsIntControlRegSpec>;
#[doc = "Register `SS_INT_CONTROL_REG` writer"]
pub type W = crate::W<SsIntControlRegSpec>;
#[doc = "Field `INTERRUPT_PRESCALE_VALUE` reader - 11:0\\]
Interrupt Prescale Value"]
pub type InterruptPrescaleValueR = crate::FieldReader<u16>;
#[doc = "Field `INTERRUPT_PRESCALE_VALUE` writer - 11:0\\]
Interrupt Prescale Value"]
pub type InterruptPrescaleValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `INTERRUPT_BYPASS_VALUE` reader - 21:16\\]
Interrupt Bypass Value"]
pub type InterruptBypassValueR = crate::FieldReader;
#[doc = "Field `INTERRUPT_BYPASS_VALUE` writer - 21:16\\]
Interrupt Bypass Value"]
pub type InterruptBypassValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INTERRUPT_SEL_VECTOR` reader - 30:30\\]
Interrupt Sel Vector Enable"]
pub type InterruptSelVectorR = crate::BitReader;
#[doc = "Field `INTERRUPT_SEL_VECTOR` writer - 30:30\\]
Interrupt Sel Vector Enable"]
pub type InterruptSelVectorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_TEST` reader - 31:31\\]
Interrupt Test"]
pub type InterruptTestR = crate::BitReader;
#[doc = "Field `INTERRUPT_TEST` writer - 31:31\\]
Interrupt Test"]
pub type InterruptTestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Interrupt Prescale Value"]
    #[inline(always)]
    pub fn interrupt_prescale_value(&self) -> InterruptPrescaleValueR {
        InterruptPrescaleValueR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Interrupt Bypass Value"]
    #[inline(always)]
    pub fn interrupt_bypass_value(&self) -> InterruptBypassValueR {
        InterruptBypassValueR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt Sel Vector Enable"]
    #[inline(always)]
    pub fn interrupt_sel_vector(&self) -> InterruptSelVectorR {
        InterruptSelVectorR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt Test"]
    #[inline(always)]
    pub fn interrupt_test(&self) -> InterruptTestR {
        InterruptTestR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Interrupt Prescale Value"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_prescale_value(&mut self) -> InterruptPrescaleValueW<SsIntControlRegSpec> {
        InterruptPrescaleValueW::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Interrupt Bypass Value"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_bypass_value(&mut self) -> InterruptBypassValueW<SsIntControlRegSpec> {
        InterruptBypassValueW::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt Sel Vector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_sel_vector(&mut self) -> InterruptSelVectorW<SsIntControlRegSpec> {
        InterruptSelVectorW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Interrupt Test"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_test(&mut self) -> InterruptTestW<SsIntControlRegSpec> {
        InterruptTestW::new(self, 31)
    }
}
#[doc = "SS Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_int_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_int_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIntControlRegSpec;
impl crate::RegisterSpec for SsIntControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_int_control_reg::R`](R) reader structure"]
impl crate::Readable for SsIntControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_int_control_reg::W`](W) writer structure"]
impl crate::Writable for SsIntControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_INT_CONTROL_REG to value 0"]
impl crate::Resettable for SsIntControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
