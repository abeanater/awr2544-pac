#[doc = "Register `STCGCR0` reader"]
pub type R = crate::R<Stcgcr0Spec>;
#[doc = "Register `STCGCR0` writer"]
pub type W = crate::W<Stcgcr0Spec>;
#[doc = "Field `RS_CNT_B1` reader - 1:0\\]
Restart/Continue or preload (RWP - Read, Priviledge Mode Write only) This bit specifies the selftest controller whether to continue the run from next interval onwards, restart from ROM address 0 or preload from a prescribed interval. This bit gets reset after the completion of selftest run. 00 = Continue NSTC run from previous interval 01 = Restart NSTC run from ROM address 0 1X = Start from segment number specified in STC_SEGPLR register"]
pub type RsCntB1R = crate::FieldReader;
#[doc = "Field `RS_CNT_B1` writer - 1:0\\]
Restart/Continue or preload (RWP - Read, Priviledge Mode Write only) This bit specifies the selftest controller whether to continue the run from next interval onwards, restart from ROM address 0 or preload from a prescribed interval. This bit gets reset after the completion of selftest run. 00 = Continue NSTC run from previous interval 01 = Restart NSTC run from ROM address 0 1X = Start from segment number specified in STC_SEGPLR register"]
pub type RsCntB1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU1` reader - 4:2\\]
Reserved bits"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 4:2\\]
Reserved bits"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCANEN_HIGH_CAP_IDLE_CYCLE` reader - 7:5\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only). *NOT BYTE ACCESSIBLE Idle Cycles between scan_en going high to func_clk_en generation and scan_en going high to misr_log_en generation. This value is used to insert that many idle cycles in the shift clock (scan_en going high to func_clk_en generation) and misr_log_clk (scan_en going high to misr_log_en generation) generation. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
pub type ScanenHighCapIdleCycleR = crate::FieldReader;
#[doc = "Field `SCANEN_HIGH_CAP_IDLE_CYCLE` writer - 7:5\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only). *NOT BYTE ACCESSIBLE Idle Cycles between scan_en going high to func_clk_en generation and scan_en going high to misr_log_en generation. This value is used to insert that many idle cycles in the shift clock (scan_en going high to func_clk_en generation) and misr_log_clk (scan_en going high to misr_log_en generation) generation. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
pub type ScanenHighCapIdleCycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAP_IDLE_CYCLE` reader - 10:8\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only) Idle Cycles before and after capture clock. This value is used to insert that many idle cycles in the Capture phase. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
pub type CapIdleCycleR = crate::FieldReader;
#[doc = "Field `CAP_IDLE_CYCLE` writer - 10:8\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only) Idle Cycles before and after capture clock. This value is used to insert that many idle cycles in the Capture phase. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
pub type CapIdleCycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU0` reader - 15:11\\]
Reserved bits"]
pub type Nu0R = crate::FieldReader;
#[doc = "Field `NU0` writer - 15:11\\]
Reserved bits"]
pub type Nu0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTCOUNT_B16` reader - 31:16\\]
Number of intervals of the self test run (RWP - Read, Priviledge Mode Write only) Count of intervals that need to be covered for a specific selftest run. The selftest controller sends out ΓÇ£completeΓÇ¥ indication once it runs all of the intervals programmed in this field. INTCOUNT_B16=0 is an invalid configuration for a selftest."]
pub type IntcountB16R = crate::FieldReader<u16>;
#[doc = "Field `INTCOUNT_B16` writer - 31:16\\]
Number of intervals of the self test run (RWP - Read, Priviledge Mode Write only) Count of intervals that need to be covered for a specific selftest run. The selftest controller sends out ΓÇ£completeΓÇ¥ indication once it runs all of the intervals programmed in this field. INTCOUNT_B16=0 is an invalid configuration for a selftest."]
pub type IntcountB16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Restart/Continue or preload (RWP - Read, Priviledge Mode Write only) This bit specifies the selftest controller whether to continue the run from next interval onwards, restart from ROM address 0 or preload from a prescribed interval. This bit gets reset after the completion of selftest run. 00 = Continue NSTC run from previous interval 01 = Restart NSTC run from ROM address 0 1X = Start from segment number specified in STC_SEGPLR register"]
    #[inline(always)]
    pub fn rs_cnt_b1(&self) -> RsCntB1R {
        RsCntB1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only). *NOT BYTE ACCESSIBLE Idle Cycles between scan_en going high to func_clk_en generation and scan_en going high to misr_log_en generation. This value is used to insert that many idle cycles in the shift clock (scan_en going high to func_clk_en generation) and misr_log_clk (scan_en going high to misr_log_en generation) generation. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
    #[inline(always)]
    pub fn scanen_high_cap_idle_cycle(&self) -> ScanenHighCapIdleCycleR {
        ScanenHighCapIdleCycleR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only) Idle Cycles before and after capture clock. This value is used to insert that many idle cycles in the Capture phase. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
    #[inline(always)]
    pub fn cap_idle_cycle(&self) -> CapIdleCycleR {
        CapIdleCycleR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu0(&self) -> Nu0R {
        Nu0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Number of intervals of the self test run (RWP - Read, Priviledge Mode Write only) Count of intervals that need to be covered for a specific selftest run. The selftest controller sends out ΓÇ£completeΓÇ¥ indication once it runs all of the intervals programmed in this field. INTCOUNT_B16=0 is an invalid configuration for a selftest."]
    #[inline(always)]
    pub fn intcount_b16(&self) -> IntcountB16R {
        IntcountB16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Restart/Continue or preload (RWP - Read, Priviledge Mode Write only) This bit specifies the selftest controller whether to continue the run from next interval onwards, restart from ROM address 0 or preload from a prescribed interval. This bit gets reset after the completion of selftest run. 00 = Continue NSTC run from previous interval 01 = Restart NSTC run from ROM address 0 1X = Start from segment number specified in STC_SEGPLR register"]
    #[inline(always)]
    #[must_use]
    pub fn rs_cnt_b1(&mut self) -> RsCntB1W<Stcgcr0Spec> {
        RsCntB1W::new(self, 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Stcgcr0Spec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only). *NOT BYTE ACCESSIBLE Idle Cycles between scan_en going high to func_clk_en generation and scan_en going high to misr_log_en generation. This value is used to insert that many idle cycles in the shift clock (scan_en going high to func_clk_en generation) and misr_log_clk (scan_en going high to misr_log_en generation) generation. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
    #[inline(always)]
    #[must_use]
    pub fn scanen_high_cap_idle_cycle(&mut self) -> ScanenHighCapIdleCycleW<Stcgcr0Spec> {
        ScanenHighCapIdleCycleW::new(self, 5)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Idle cycles before and after capture clock (RWP - Read, Priviledge Mode Write only) Idle Cycles before and after capture clock. This value is used to insert that many idle cycles in the Capture phase. Programmable idle cycles allow implementation flexibility on SCAN_EN signal at chip level based on the size of the UUT and timing requirements."]
    #[inline(always)]
    #[must_use]
    pub fn cap_idle_cycle(&mut self) -> CapIdleCycleW<Stcgcr0Spec> {
        CapIdleCycleW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu0(&mut self) -> Nu0W<Stcgcr0Spec> {
        Nu0W::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Number of intervals of the self test run (RWP - Read, Priviledge Mode Write only) Count of intervals that need to be covered for a specific selftest run. The selftest controller sends out ΓÇ£completeΓÇ¥ indication once it runs all of the intervals programmed in this field. INTCOUNT_B16=0 is an invalid configuration for a selftest."]
    #[inline(always)]
    #[must_use]
    pub fn intcount_b16(&mut self) -> IntcountB16W<Stcgcr0Spec> {
        IntcountB16W::new(self, 16)
    }
}
#[doc = "Self test Global control Reg0. *NOT BYTE ACCESSIBLE\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcgcr0Spec;
impl crate::RegisterSpec for Stcgcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcgcr0::R`](R) reader structure"]
impl crate::Readable for Stcgcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`stcgcr0::W`](W) writer structure"]
impl crate::Writable for Stcgcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCGCR0 to value 0"]
impl crate::Resettable for Stcgcr0Spec {
    const RESET_VALUE: u32 = 0;
}
