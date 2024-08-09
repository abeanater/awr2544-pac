#[doc = "Register `DMMSWINT1` reader"]
pub type R = crate::R<Dmmswint1Spec>;
#[doc = "Register `DMMSWINT1` writer"]
pub type W = crate::W<Dmmswint1Spec>;
#[doc = "Field `DMMADCBUFPINPONSEL` reader - 16:16\\]
ADC Buffer Ping Pong select for HIL Mode"]
pub type DmmadcbufpinponselR = crate::BitReader;
#[doc = "Field `DMMADCBUFPINPONSEL` writer - 16:16\\]
ADC Buffer Ping Pong select for HIL Mode"]
pub type DmmadcbufpinponselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMMADCBUFWREN` reader - 17:17\\]
ADC Buffer Write Enable from DMM. 0 --> Write to ADC BUF memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from ADCBUF_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMADCBUFPINPONSEL register."]
pub type DmmadcbufwrenR = crate::BitReader;
#[doc = "Field `DMMADCBUFWREN` writer - 17:17\\]
ADC Buffer Write Enable from DMM. 0 --> Write to ADC BUF memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from ADCBUF_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMADCBUFPINPONSEL register."]
pub type DmmadcbufwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMMCPWREN` reader - 18:18\\]
Writiing 1'b1: Enables DMM writes in to CP read registers 1'b0: Disables DMM writes to CP read registers"]
pub type DmmcpwrenR = crate::BitReader;
#[doc = "Field `DMMCPWREN` writer - 18:18\\]
Writiing 1'b1: Enables DMM writes in to CP read registers 1'b0: Disables DMM writes to CP read registers"]
pub type DmmcpwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMMCQPINPONSEL` reader - 21:21\\]
CQ Ping Pong select for HIL Mode"]
pub type DmmcqpinponselR = crate::BitReader;
#[doc = "Field `DMMCQPINPONSEL` writer - 21:21\\]
CQ Ping Pong select for HIL Mode"]
pub type DmmcqpinponselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMMCQWREN` reader - 22:22\\]
CQ Write Enable from DMM. 0 --> Write to CQ memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from CQ_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMCQPINPONSEL register."]
pub type DmmcqwrenR = crate::BitReader;
#[doc = "Field `DMMCQWREN` writer - 22:22\\]
CQ Write Enable from DMM. 0 --> Write to CQ memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from CQ_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMCQPINPONSEL register."]
pub type DmmcqwrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
ADC Buffer Ping Pong select for HIL Mode"]
    #[inline(always)]
    pub fn dmmadcbufpinponsel(&self) -> DmmadcbufpinponselR {
        DmmadcbufpinponselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
ADC Buffer Write Enable from DMM. 0 --> Write to ADC BUF memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from ADCBUF_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMADCBUFPINPONSEL register."]
    #[inline(always)]
    pub fn dmmadcbufwren(&self) -> DmmadcbufwrenR {
        DmmadcbufwrenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writiing 1'b1: Enables DMM writes in to CP read registers 1'b0: Disables DMM writes to CP read registers"]
    #[inline(always)]
    pub fn dmmcpwren(&self) -> DmmcpwrenR {
        DmmcpwrenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
CQ Ping Pong select for HIL Mode"]
    #[inline(always)]
    pub fn dmmcqpinponsel(&self) -> DmmcqpinponselR {
        DmmcqpinponselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
CQ Write Enable from DMM. 0 --> Write to CQ memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from CQ_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMCQPINPONSEL register."]
    #[inline(always)]
    pub fn dmmcqwren(&self) -> DmmcqwrenR {
        DmmcqwrenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
ADC Buffer Ping Pong select for HIL Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmmadcbufpinponsel(&mut self) -> DmmadcbufpinponselW<Dmmswint1Spec> {
        DmmadcbufpinponselW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
ADC Buffer Write Enable from DMM. 0 --> Write to ADC BUF memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from ADCBUF_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMADCBUFPINPONSEL register."]
    #[inline(always)]
    #[must_use]
    pub fn dmmadcbufwren(&mut self) -> DmmadcbufwrenW<Dmmswint1Spec> {
        DmmadcbufwrenW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Writiing 1'b1: Enables DMM writes in to CP read registers 1'b0: Disables DMM writes to CP read registers"]
    #[inline(always)]
    #[must_use]
    pub fn dmmcpwren(&mut self) -> DmmcpwrenW<Dmmswint1Spec> {
        DmmcpwrenW::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
CQ Ping Pong select for HIL Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmmcqpinponsel(&mut self) -> DmmcqpinponselW<Dmmswint1Spec> {
        DmmcqpinponselW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
CQ Write Enable from DMM. 0 --> Write to CQ memory will happen from DFE and Ping-pong select will come from HW FSM (same as ADC Buffer ping-pong select). 1 --> Write to CQ memory will happen from CQ_W slave port in DSS interconnect using DMM as master and Ping-pong select will come from DMMCQPINPONSEL register."]
    #[inline(always)]
    #[must_use]
    pub fn dmmcqwren(&mut self) -> DmmcqwrenW<Dmmswint1Spec> {
        DmmcqwrenW::new(self, 22)
    }
}
#[doc = "DMMSWINT1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmswint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmswint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmmswint1Spec;
impl crate::RegisterSpec for Dmmswint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmswint1::R`](R) reader structure"]
impl crate::Readable for Dmmswint1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmmswint1::W`](W) writer structure"]
impl crate::Writable for Dmmswint1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMMSWINT1 to value 0"]
impl crate::Resettable for Dmmswint1Spec {
    const RESET_VALUE: u32 = 0;
}
