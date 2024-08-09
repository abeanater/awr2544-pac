#[doc = "Register `aggr_status_clr` reader"]
pub type R = crate::R<AggrStatusClrSpec>;
#[doc = "Register `aggr_status_clr` writer"]
pub type W = crate::W<AggrStatusClrSpec>;
#[doc = "Field `INTERRUPT_STATUS_CLEAR` reader - 1:0\\]
interrupt status clear for parity errors"]
pub type InterruptStatusClearR = crate::FieldReader;
#[doc = "Field `INTERRUPT_STATUS_CLEAR` writer - 1:0\\]
interrupt status clear for parity errors"]
pub type InterruptStatusClearW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERRUPT_STATUS_CLEAR` reader - 3:2\\]
interrupt status clear for svbus timeout errors"]
pub type InterruptStatusClearR = crate::FieldReader;
#[doc = "Field `INTERRUPT_STATUS_CLEAR` writer - 3:2\\]
interrupt status clear for svbus timeout errors"]
pub type InterruptStatusClearW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status clear for parity errors"]
    #[inline(always)]
    pub fn interrupt_status_clear(&self) -> InterruptStatusClearR {
        InterruptStatusClearR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status clear for svbus timeout errors"]
    #[inline(always)]
    pub fn interrupt_status_clear(&self) -> InterruptStatusClearR {
        InterruptStatusClearR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status clear for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_status_clear(&mut self) -> InterruptStatusClearW<AggrStatusClrSpec> {
        InterruptStatusClearW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status clear for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_status_clear(&mut self) -> InterruptStatusClearW<AggrStatusClrSpec> {
        InterruptStatusClearW::new(self, 2)
    }
}
#[doc = "AGGR interrupt status clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrStatusClrSpec;
impl crate::RegisterSpec for AggrStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_status_clr::R`](R) reader structure"]
impl crate::Readable for AggrStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_status_clr::W`](W) writer structure"]
impl crate::Writable for AggrStatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aggr_status_clr to value 0"]
impl crate::Resettable for AggrStatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
