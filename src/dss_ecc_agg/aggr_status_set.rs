#[doc = "Register `aggr_status_set` reader"]
pub type R = crate::R<AggrStatusSetSpec>;
#[doc = "Register `aggr_status_set` writer"]
pub type W = crate::W<AggrStatusSetSpec>;
#[doc = "Field `INTERRUPT_STATUS_SET_1` reader - 1:0\\]
interrupt status set for parity errors"]
pub type InterruptStatusSet1R = crate::FieldReader;
#[doc = "Field `INTERRUPT_STATUS_SET_1` writer - 1:0\\]
interrupt status set for parity errors"]
pub type InterruptStatusSet1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERRUPT_STATUS_SET` reader - 3:2\\]
interrupt status set for svbus timeout errors"]
pub type InterruptStatusSetR = crate::FieldReader;
#[doc = "Field `INTERRUPT_STATUS_SET` writer - 3:2\\]
interrupt status set for svbus timeout errors"]
pub type InterruptStatusSetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors"]
    #[inline(always)]
    pub fn interrupt_status_set_1(&self) -> InterruptStatusSet1R {
        InterruptStatusSet1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors"]
    #[inline(always)]
    pub fn interrupt_status_set(&self) -> InterruptStatusSetR {
        InterruptStatusSetR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_status_set_1(&mut self) -> InterruptStatusSet1W<AggrStatusSetSpec> {
        InterruptStatusSet1W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_status_set(&mut self) -> InterruptStatusSetW<AggrStatusSetSpec> {
        InterruptStatusSetW::new(self, 2)
    }
}
#[doc = "AGGR interrupt status set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrStatusSetSpec;
impl crate::RegisterSpec for AggrStatusSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_status_set::R`](R) reader structure"]
impl crate::Readable for AggrStatusSetSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_status_set::W`](W) writer structure"]
impl crate::Writable for AggrStatusSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aggr_status_set to value 0"]
impl crate::Resettable for AggrStatusSetSpec {
    const RESET_VALUE: u32 = 0;
}
