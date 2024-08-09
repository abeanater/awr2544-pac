#[doc = "Register `LOOP_CNT` reader"]
pub type R = crate::R<LoopCntSpec>;
#[doc = "Register `LOOP_CNT` writer"]
pub type W = crate::W<LoopCntSpec>;
#[doc = "Field `loop_cnt` reader - 11:0\\]
Loop count"]
pub type LoopCntR = crate::FieldReader<u16>;
#[doc = "Field `loop_cnt` writer - 11:0\\]
Loop count"]
pub type LoopCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Loop count"]
    #[inline(always)]
    pub fn loop_cnt(&self) -> LoopCntR {
        LoopCntR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Loop count"]
    #[inline(always)]
    #[must_use]
    pub fn loop_cnt(&mut self) -> LoopCntW<LoopCntSpec> {
        LoopCntW::new(self, 0)
    }
}
#[doc = "LOOP_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopCntSpec;
impl crate::RegisterSpec for LoopCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop_cnt::R`](R) reader structure"]
impl crate::Readable for LoopCntSpec {}
#[doc = "`write(|w| ..)` method takes [`loop_cnt::W`](W) writer structure"]
impl crate::Writable for LoopCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOP_CNT to value 0"]
impl crate::Resettable for LoopCntSpec {
    const RESET_VALUE: u32 = 0;
}
