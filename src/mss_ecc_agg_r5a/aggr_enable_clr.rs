#[doc = "Register `aggr_enable_clr` reader"]
pub type R = crate::R<AggrEnableClrSpec>;
#[doc = "Register `aggr_enable_clr` writer"]
pub type W = crate::W<AggrEnableClrSpec>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR_1` reader - 0:0\\]
interrupt enable clear for parity errors"]
pub type InterruptEnableClear1R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR_1` writer - 0:0\\]
interrupt enable clear for parity errors"]
pub type InterruptEnableClear1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 1:1\\]
interrupt enable clear for svbus timeout errors"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 1:1\\]
interrupt enable clear for svbus timeout errors"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors"]
    #[inline(always)]
    pub fn interrupt_enable_clear_1(&self) -> InterruptEnableClear1R {
        InterruptEnableClear1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear_1(&mut self) -> InterruptEnableClear1W<AggrEnableClrSpec> {
        InterruptEnableClear1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<AggrEnableClrSpec> {
        InterruptEnableClearW::new(self, 1)
    }
}
#[doc = "AGGR interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableClrSpec;
impl crate::RegisterSpec for AggrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_clr::R`](R) reader structure"]
impl crate::Readable for AggrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_clr::W`](W) writer structure"]
impl crate::Writable for AggrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aggr_enable_clr to value 0"]
impl crate::Resettable for AggrEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
