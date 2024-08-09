#[doc = "Register `PARAM_RAM_LOOP` reader"]
pub type R = crate::R<ParamRamLoopSpec>;
#[doc = "Register `PARAM_RAM_LOOP` writer"]
pub type W = crate::W<ParamRamLoopSpec>;
#[doc = "Field `numloops` reader - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
pub type NumloopsR = crate::FieldReader<u16>;
#[doc = "Field `numloops` writer - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
pub type NumloopsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
    #[inline(always)]
    pub fn numloops(&self) -> NumloopsR {
        NumloopsR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn numloops(&mut self) -> NumloopsW<ParamRamLoopSpec> {
        NumloopsW::new(self, 0)
    }
}
#[doc = "PARAM_RAM_LOOP\n\nYou can [`read`](crate::Reg::read) this register and get [`param_ram_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_ram_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamRamLoopSpec;
impl crate::RegisterSpec for ParamRamLoopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param_ram_loop::R`](R) reader structure"]
impl crate::Readable for ParamRamLoopSpec {}
#[doc = "`write(|w| ..)` method takes [`param_ram_loop::W`](W) writer structure"]
impl crate::Writable for ParamRamLoopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM_RAM_LOOP to value 0"]
impl crate::Resettable for ParamRamLoopSpec {
    const RESET_VALUE: u32 = 0;
}
