#[doc = "Register `TICKCNT` reader"]
pub type R = crate::R<TickcntSpec>;
#[doc = "Register `TICKCNT` writer"]
pub type W = crate::W<TickcntSpec>;
#[doc = "Field `TICKVALUE` reader - 15:0\\]
Initial value for tick counter. TICKVALUE stores the initial value for the tick counter. The tick counter is loaded with TICKVALUE every time an under-flow condition occurs and every time the RELOAD flag is set by the host"]
pub type TickvalueR = crate::FieldReader<u16>;
#[doc = "Field `TICKVALUE` writer - 15:0\\]
Initial value for tick counter. TICKVALUE stores the initial value for the tick counter. The tick counter is loaded with TICKVALUE every time an under-flow condition occurs and every time the RELOAD flag is set by the host"]
pub type TickvalueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 27:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 27:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CLKCTRL` reader - 29:28\\]
Tick counter clock source control. CLKCTRL\\[1:0\\]
defines the clock source that is used to clock the MibSPI internal tick counter.CLKCTRL\\[1:0\\]
Description 00b SPICLK of Data word format 0 is selected as clock source of tick counter 01b SPICLK of Data word format 1 is selected as clock source of tick counter 10b SPICLK of Data word format 2 is selected as clock source of tick counter 11b SPICLK of Data word format 3 is selected as clock source of tick counter"]
pub type ClkctrlR = crate::FieldReader;
#[doc = "Field `CLKCTRL` writer - 29:28\\]
Tick counter clock source control. CLKCTRL\\[1:0\\]
defines the clock source that is used to clock the MibSPI internal tick counter.CLKCTRL\\[1:0\\]
Description 00b SPICLK of Data word format 0 is selected as clock source of tick counter 01b SPICLK of Data word format 1 is selected as clock source of tick counter 10b SPICLK of Data word format 2 is selected as clock source of tick counter 11b SPICLK of Data word format 3 is selected as clock source of tick counter"]
pub type ClkctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RELOAD` reader - 30:30\\]
Re-load tick counter. RELOAD is a set-only bit, i.e. writing a ΓÇ£1ΓÇ¥ to it automatically reloads the Tick Counter with the value stored in TICKVALUE. Reading RELOAD always returns a ΓÇ£0ΓÇ¥. Note: When the tick counter is reloaded by the RELOAD bit, the trigger signal is not toggled."]
pub type ReloadR = crate::BitReader;
#[doc = "Field `RELOAD` writer - 30:30\\]
Re-load tick counter. RELOAD is a set-only bit, i.e. writing a ΓÇ£1ΓÇ¥ to it automatically reloads the Tick Counter with the value stored in TICKVALUE. Reading RELOAD always returns a ΓÇ£0ΓÇ¥. Note: When the tick counter is reloaded by the RELOAD bit, the trigger signal is not toggled."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKENA` reader - 31:31\\]
Tick counter enable. 1 =The MibSPI internal tick counter is enabled and is clocked by the clock source selected by CLKCTRL\\[1:0\\]. When the tick counter is enabled it starts down-counting from its current value. When TICKENA goes from ΓÇ£0ΓÇ¥ to ΓÇ£1ΓÇ¥ the tick counter is automatically loaded with the TICKVALUE. 0 =The MibSPI internal tick counter is disabled. The counter value remains unchanged. Note: When the tick counter is disabled the trigger signal is forced low."]
pub type TickenaR = crate::BitReader;
#[doc = "Field `TICKENA` writer - 31:31\\]
Tick counter enable. 1 =The MibSPI internal tick counter is enabled and is clocked by the clock source selected by CLKCTRL\\[1:0\\]. When the tick counter is enabled it starts down-counting from its current value. When TICKENA goes from ΓÇ£0ΓÇ¥ to ΓÇ£1ΓÇ¥ the tick counter is automatically loaded with the TICKVALUE. 0 =The MibSPI internal tick counter is disabled. The counter value remains unchanged. Note: When the tick counter is disabled the trigger signal is forced low."]
pub type TickenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Initial value for tick counter. TICKVALUE stores the initial value for the tick counter. The tick counter is loaded with TICKVALUE every time an under-flow condition occurs and every time the RELOAD flag is set by the host"]
    #[inline(always)]
    pub fn tickvalue(&self) -> TickvalueR {
        TickvalueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Tick counter clock source control. CLKCTRL\\[1:0\\]
defines the clock source that is used to clock the MibSPI internal tick counter.CLKCTRL\\[1:0\\]
Description 00b SPICLK of Data word format 0 is selected as clock source of tick counter 01b SPICLK of Data word format 1 is selected as clock source of tick counter 10b SPICLK of Data word format 2 is selected as clock source of tick counter 11b SPICLK of Data word format 3 is selected as clock source of tick counter"]
    #[inline(always)]
    pub fn clkctrl(&self) -> ClkctrlR {
        ClkctrlR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Re-load tick counter. RELOAD is a set-only bit, i.e. writing a ΓÇ£1ΓÇ¥ to it automatically reloads the Tick Counter with the value stored in TICKVALUE. Reading RELOAD always returns a ΓÇ£0ΓÇ¥. Note: When the tick counter is reloaded by the RELOAD bit, the trigger signal is not toggled."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Tick counter enable. 1 =The MibSPI internal tick counter is enabled and is clocked by the clock source selected by CLKCTRL\\[1:0\\]. When the tick counter is enabled it starts down-counting from its current value. When TICKENA goes from ΓÇ£0ΓÇ¥ to ΓÇ£1ΓÇ¥ the tick counter is automatically loaded with the TICKVALUE. 0 =The MibSPI internal tick counter is disabled. The counter value remains unchanged. Note: When the tick counter is disabled the trigger signal is forced low."]
    #[inline(always)]
    pub fn tickena(&self) -> TickenaR {
        TickenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Initial value for tick counter. TICKVALUE stores the initial value for the tick counter. The tick counter is loaded with TICKVALUE every time an under-flow condition occurs and every time the RELOAD flag is set by the host"]
    #[inline(always)]
    #[must_use]
    pub fn tickvalue(&mut self) -> TickvalueW<TickcntSpec> {
        TickvalueW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<TickcntSpec> {
        NuW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Tick counter clock source control. CLKCTRL\\[1:0\\]
defines the clock source that is used to clock the MibSPI internal tick counter.CLKCTRL\\[1:0\\]
Description 00b SPICLK of Data word format 0 is selected as clock source of tick counter 01b SPICLK of Data word format 1 is selected as clock source of tick counter 10b SPICLK of Data word format 2 is selected as clock source of tick counter 11b SPICLK of Data word format 3 is selected as clock source of tick counter"]
    #[inline(always)]
    #[must_use]
    pub fn clkctrl(&mut self) -> ClkctrlW<TickcntSpec> {
        ClkctrlW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Re-load tick counter. RELOAD is a set-only bit, i.e. writing a ΓÇ£1ΓÇ¥ to it automatically reloads the Tick Counter with the value stored in TICKVALUE. Reading RELOAD always returns a ΓÇ£0ΓÇ¥. Note: When the tick counter is reloaded by the RELOAD bit, the trigger signal is not toggled."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<TickcntSpec> {
        ReloadW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Tick counter enable. 1 =The MibSPI internal tick counter is enabled and is clocked by the clock source selected by CLKCTRL\\[1:0\\]. When the tick counter is enabled it starts down-counting from its current value. When TICKENA goes from ΓÇ£0ΓÇ¥ to ΓÇ£1ΓÇ¥ the tick counter is automatically loaded with the TICKVALUE. 0 =The MibSPI internal tick counter is disabled. The counter value remains unchanged. Note: When the tick counter is disabled the trigger signal is forced low."]
    #[inline(always)]
    #[must_use]
    pub fn tickena(&mut self) -> TickenaW<TickcntSpec> {
        TickenaW::new(self, 31)
    }
}
#[doc = "Tick Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tickcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tickcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TickcntSpec;
impl crate::RegisterSpec for TickcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tickcnt::R`](R) reader structure"]
impl crate::Readable for TickcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tickcnt::W`](W) writer structure"]
impl crate::Writable for TickcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TICKCNT to value 0"]
impl crate::Resettable for TickcntSpec {
    const RESET_VALUE: u32 = 0;
}
