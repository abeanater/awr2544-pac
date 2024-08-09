#[doc = "Register `PARAM_RAM_IDX` reader"]
pub type R = crate::R<ParamRamIdxSpec>;
#[doc = "Register `PARAM_RAM_IDX` writer"]
pub type W = crate::W<ParamRamIdxSpec>;
#[doc = "Field `param_start_idx` reader - 9:0\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX.."]
pub type ParamStartIdxR = crate::FieldReader<u16>;
#[doc = "Field `param_start_idx` writer - 9:0\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX.."]
pub type ParamStartIdxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `param_end_idx` reader - 25:16\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX."]
pub type ParamEndIdxR = crate::FieldReader<u16>;
#[doc = "Field `param_end_idx` writer - 25:16\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX."]
pub type ParamEndIdxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX.."]
    #[inline(always)]
    pub fn param_start_idx(&self) -> ParamStartIdxR {
        ParamStartIdxR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX."]
    #[inline(always)]
    pub fn param_end_idx(&self) -> ParamEndIdxR {
        ParamEndIdxR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX.."]
    #[inline(always)]
    #[must_use]
    pub fn param_start_idx(&mut self) -> ParamStartIdxW<ParamRamIdxSpec> {
        ParamStartIdxW::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
The state machine starts at the parameter-set specified by PARAM_START_IDX and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_ENS_IDX, it loops back to the start index as specified by PARAM_START_IDX."]
    #[inline(always)]
    #[must_use]
    pub fn param_end_idx(&mut self) -> ParamEndIdxW<ParamRamIdxSpec> {
        ParamEndIdxW::new(self, 16)
    }
}
#[doc = "PARAM_RAM_IDX\n\nYou can [`read`](crate::Reg::read) this register and get [`param_ram_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_ram_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamRamIdxSpec;
impl crate::RegisterSpec for ParamRamIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param_ram_idx::R`](R) reader structure"]
impl crate::Readable for ParamRamIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`param_ram_idx::W`](W) writer structure"]
impl crate::Writable for ParamRamIdxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM_RAM_IDX to value 0"]
impl crate::Resettable for ParamRamIdxSpec {
    const RESET_VALUE: u32 = 0;
}
