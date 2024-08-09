#[doc = "Register `aggr_enable_set` reader"]
pub type R = crate::R<AggrEnableSetSpec>;
#[doc = "Register `aggr_enable_set` writer"]
pub type W = crate::W<AggrEnableSetSpec>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 0:0\\]
interrupt enable set for parity errors"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 0:0\\]
interrupt enable set for parity errors"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 1:1\\]
interrupt enable set for svbus timeout errors"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 1:1\\]
interrupt enable set for svbus timeout errors"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable set for parity errors"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable set for svbus timeout errors"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable set for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<AggrEnableSetSpec> {
        InterruptEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable set for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<AggrEnableSetSpec> {
        InterruptEnableSetW::new(self, 1)
    }
}
#[doc = "AGGR interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableSetSpec;
impl crate::RegisterSpec for AggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for AggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for AggrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aggr_enable_set to value 0"]
impl crate::Resettable for AggrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
