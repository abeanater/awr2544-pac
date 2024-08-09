#[doc = "Register `CPTS_TS_COMP_NUDGE_REG` reader"]
pub type R = crate::R<CptsTsCompNudgeRegSpec>;
#[doc = "Register `CPTS_TS_COMP_NUDGE_REG` writer"]
pub type W = crate::W<CptsTsCompNudgeRegSpec>;
#[doc = "Field `THIS_2S_COMPLEMENT` reader - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
pub type This2sComplementR = crate::FieldReader;
#[doc = "Field `THIS_2S_COMPLEMENT` writer - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
pub type This2sComplementW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
    #[inline(always)]
    pub fn this_2s_complement(&self) -> This2sComplementR {
        This2sComplementR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
    #[inline(always)]
    #[must_use]
    pub fn this_2s_complement(&mut self) -> This2sComplementW<CptsTsCompNudgeRegSpec> {
        This2sComplementW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Nudge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_comp_nudge_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_comp_nudge_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsCompNudgeRegSpec;
impl crate::RegisterSpec for CptsTsCompNudgeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_comp_nudge_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsCompNudgeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_comp_nudge_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsCompNudgeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_COMP_NUDGE_REG to value 0"]
impl crate::Resettable for CptsTsCompNudgeRegSpec {
    const RESET_VALUE: u32 = 0;
}
