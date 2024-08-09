#[doc = "Register `CQCFG2` reader"]
pub type R = crate::R<Cqcfg2Spec>;
#[doc = "Register `CQCFG2` writer"]
pub type W = crate::W<Cqcfg2Spec>;
#[doc = "Field `CQ0TESTMODEEN` reader - 0:0\\]
TI Internal Feaure Test Mode enable for CQ0 (WBE). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
pub type Cq0testmodeenR = crate::BitReader;
#[doc = "Field `CQ0TESTMODEEN` writer - 0:0\\]
TI Internal Feaure Test Mode enable for CQ0 (WBE). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
pub type Cq0testmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQ1TESTMODEEN` reader - 4:4\\]
TI Internal Feaure Test Mode enable for CQ1 (SI). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
pub type Cq1testmodeenR = crate::BitReader;
#[doc = "Field `CQ1TESTMODEEN` writer - 4:4\\]
TI Internal Feaure Test Mode enable for CQ1 (SI). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
pub type Cq1testmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQ2TESTMODEEN` reader - 8:8\\]
TI Internal Feaure Test Mode enable for CQ2 (ADC/RxIF Saturation). Once enabled, each 8 bit data is same as Addr+1."]
pub type Cq2testmodeenR = crate::BitReader;
#[doc = "Field `CQ2TESTMODEEN` writer - 8:8\\]
TI Internal Feaure Test Mode enable for CQ2 (ADC/RxIF Saturation). Once enabled, each 8 bit data is same as Addr+1."]
pub type Cq2testmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPIPOSELCNT` reader - 12:12\\]
Ping pong select override control for CQ Memory. 0 --> Ping-pong select comes from HW FSM (same as the ping-pong select for ADC Buffer)/DMMCQPINPONSEL 1 --> Ping pong select for CQ memory is taken from SW register (CQPIPOSELVAL)"]
pub type CqpiposelcntR = crate::BitReader;
#[doc = "Field `CQPIPOSELCNT` writer - 12:12\\]
Ping pong select override control for CQ Memory. 0 --> Ping-pong select comes from HW FSM (same as the ping-pong select for ADC Buffer)/DMMCQPINPONSEL 1 --> Ping pong select for CQ memory is taken from SW register (CQPIPOSELVAL)"]
pub type CqpiposelcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPIPOSELVAL` reader - 16:16\\]
Ping pong select override value for CQ Memory. 1 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to ping memory and write access from CQ_W/DFE write will be routed to pong memory. 0 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to pong memory and write access from CQ_W/DFE write will be routed to ping memory."]
pub type CqpiposelvalR = crate::BitReader;
#[doc = "Field `CQPIPOSELVAL` writer - 16:16\\]
Ping pong select override value for CQ Memory. 1 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to ping memory and write access from CQ_W/DFE write will be routed to pong memory. 0 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to pong memory and write access from CQ_W/DFE write will be routed to ping memory."]
pub type CqpiposelvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQ_CLK_GATE` reader - 17:17\\]
writing: 1'b0: ungates the clk to CQ logic 1'b1: Gates the clk to CQ logic"]
pub type CqClkGateR = crate::BitReader;
#[doc = "Field `CQ_CLK_GATE` writer - 17:17\\]
writing: 1'b0: ungates the clk to CQ logic 1'b1: Gates the clk to CQ logic"]
pub type CqClkGateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feaure Test Mode enable for CQ0 (WBE). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
    #[inline(always)]
    pub fn cq0testmodeen(&self) -> Cq0testmodeenR {
        Cq0testmodeenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feaure Test Mode enable for CQ1 (SI). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
    #[inline(always)]
    pub fn cq1testmodeen(&self) -> Cq1testmodeenR {
        Cq1testmodeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal Feaure Test Mode enable for CQ2 (ADC/RxIF Saturation). Once enabled, each 8 bit data is same as Addr+1."]
    #[inline(always)]
    pub fn cq2testmodeen(&self) -> Cq2testmodeenR {
        Cq2testmodeenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping pong select override control for CQ Memory. 0 --> Ping-pong select comes from HW FSM (same as the ping-pong select for ADC Buffer)/DMMCQPINPONSEL 1 --> Ping pong select for CQ memory is taken from SW register (CQPIPOSELVAL)"]
    #[inline(always)]
    pub fn cqpiposelcnt(&self) -> CqpiposelcntR {
        CqpiposelcntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Ping pong select override value for CQ Memory. 1 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to ping memory and write access from CQ_W/DFE write will be routed to pong memory. 0 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to pong memory and write access from CQ_W/DFE write will be routed to ping memory."]
    #[inline(always)]
    pub fn cqpiposelval(&self) -> CqpiposelvalR {
        CqpiposelvalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
writing: 1'b0: ungates the clk to CQ logic 1'b1: Gates the clk to CQ logic"]
    #[inline(always)]
    pub fn cq_clk_gate(&self) -> CqClkGateR {
        CqClkGateR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feaure Test Mode enable for CQ0 (WBE). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn cq0testmodeen(&mut self) -> Cq0testmodeenW<Cqcfg2Spec> {
        Cq0testmodeenW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feaure Test Mode enable for CQ1 (SI). Once enabled, each 16 bit data is same as \\[2*Addr+1 for the MSB 8 bits and Addr+1 for the LSB 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn cq1testmodeen(&mut self) -> Cq1testmodeenW<Cqcfg2Spec> {
        Cq1testmodeenW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal Feaure Test Mode enable for CQ2 (ADC/RxIF Saturation). Once enabled, each 8 bit data is same as Addr+1."]
    #[inline(always)]
    #[must_use]
    pub fn cq2testmodeen(&mut self) -> Cq2testmodeenW<Cqcfg2Spec> {
        Cq2testmodeenW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Ping pong select override control for CQ Memory. 0 --> Ping-pong select comes from HW FSM (same as the ping-pong select for ADC Buffer)/DMMCQPINPONSEL 1 --> Ping pong select for CQ memory is taken from SW register (CQPIPOSELVAL)"]
    #[inline(always)]
    #[must_use]
    pub fn cqpiposelcnt(&mut self) -> CqpiposelcntW<Cqcfg2Spec> {
        CqpiposelcntW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Ping pong select override value for CQ Memory. 1 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to ping memory and write access from CQ_W/DFE write will be routed to pong memory. 0 --> Read access from Chirp Info Slave of DSS Interconnect will be routed to pong memory and write access from CQ_W/DFE write will be routed to ping memory."]
    #[inline(always)]
    #[must_use]
    pub fn cqpiposelval(&mut self) -> CqpiposelvalW<Cqcfg2Spec> {
        CqpiposelvalW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
writing: 1'b0: ungates the clk to CQ logic 1'b1: Gates the clk to CQ logic"]
    #[inline(always)]
    #[must_use]
    pub fn cq_clk_gate(&mut self) -> CqClkGateW<Cqcfg2Spec> {
        CqClkGateW::new(self, 17)
    }
}
#[doc = "CQCFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqcfg2Spec;
impl crate::RegisterSpec for Cqcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg2::R`](R) reader structure"]
impl crate::Readable for Cqcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cqcfg2::W`](W) writer structure"]
impl crate::Writable for Cqcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCFG2 to value 0"]
impl crate::Resettable for Cqcfg2Spec {
    const RESET_VALUE: u32 = 0;
}
