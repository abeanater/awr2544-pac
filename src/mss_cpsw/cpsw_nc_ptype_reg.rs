#[doc = "Register `CPSW_NC_PTYPE_REG` reader"]
pub type R = crate::R<CpswNcPtypeRegSpec>;
#[doc = "Register `CPSW_NC_PTYPE_REG` writer"]
pub type W = crate::W<CpswNcPtypeRegSpec>;
#[doc = "Field `ESCALATE_PRIORITY_LOAD` reader - 4:0\\]
Escalate Priority Load Value"]
pub type EscalatePriorityLoadR = crate::FieldReader;
#[doc = "Field `ESCALATE_PRIORITY_LOAD` writer - 4:0\\]
Escalate Priority Load Value"]
pub type EscalatePriorityLoadW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_0_PRIORITY` reader - 8:8\\]
Port 0 Priority Type Escalate"]
pub type Port0PriorityR = crate::BitReader;
#[doc = "Field `PORT_0_PRIORITY` writer - 8:8\\]
Port 0 Priority Type Escalate"]
pub type Port0PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_1_PRIORITY` reader - 9:9\\]
Port 1 Priority Type Escalate"]
pub type Port1PriorityR = crate::BitReader;
#[doc = "Field `PORT_1_PRIORITY` writer - 9:9\\]
Port 1 Priority Type Escalate"]
pub type Port1PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_2_PRIORITY` reader - 10:10\\]
Port 2 Priority Type Escalate"]
pub type Port2PriorityR = crate::BitReader;
#[doc = "Field `PORT_2_PRIORITY` writer - 10:10\\]
Port 2 Priority Type Escalate"]
pub type Port2PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_3_PRIORITY` reader - 11:11\\]
Port 3 Priority Type Escalate"]
pub type Port3PriorityR = crate::BitReader;
#[doc = "Field `PORT_3_PRIORITY` writer - 11:11\\]
Port 3 Priority Type Escalate"]
pub type Port3PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_4_PRIORITY` reader - 12:12\\]
Port 4 Priority Type Escalate"]
pub type Port4PriorityR = crate::BitReader;
#[doc = "Field `PORT_4_PRIORITY` writer - 12:12\\]
Port 4 Priority Type Escalate"]
pub type Port4PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_5_PRIORITY` reader - 13:13\\]
Port 5 Priority Type Escalate"]
pub type Port5PriorityR = crate::BitReader;
#[doc = "Field `PORT_5_PRIORITY` writer - 13:13\\]
Port 5 Priority Type Escalate"]
pub type Port5PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_6_PRIORITY` reader - 14:14\\]
Port 6 Priority Type Escalate"]
pub type Port6PriorityR = crate::BitReader;
#[doc = "Field `PORT_6_PRIORITY` writer - 14:14\\]
Port 6 Priority Type Escalate"]
pub type Port6PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_7_PRIORITY` reader - 15:15\\]
Port 7 Priority Type Escalate"]
pub type Port7PriorityR = crate::BitReader;
#[doc = "Field `PORT_7_PRIORITY` writer - 15:15\\]
Port 7 Priority Type Escalate"]
pub type Port7PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_8_PRIORITY` reader - 16:16\\]
Port 8 Priority Type Escalate"]
pub type Port8PriorityR = crate::BitReader;
#[doc = "Field `PORT_8_PRIORITY` writer - 16:16\\]
Port 8 Priority Type Escalate"]
pub type Port8PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Escalate Priority Load Value"]
    #[inline(always)]
    pub fn escalate_priority_load(&self) -> EscalatePriorityLoadR {
        EscalatePriorityLoadR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 0 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_0_priority(&self) -> Port0PriorityR {
        Port0PriorityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 1 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_1_priority(&self) -> Port1PriorityR {
        Port1PriorityR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 2 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_2_priority(&self) -> Port2PriorityR {
        Port2PriorityR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 3 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_3_priority(&self) -> Port3PriorityR {
        Port3PriorityR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 4 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_4_priority(&self) -> Port4PriorityR {
        Port4PriorityR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 5 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_5_priority(&self) -> Port5PriorityR {
        Port5PriorityR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 6 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_6_priority(&self) -> Port6PriorityR {
        Port6PriorityR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 7 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_7_priority(&self) -> Port7PriorityR {
        Port7PriorityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Port 8 Priority Type Escalate"]
    #[inline(always)]
    pub fn port_8_priority(&self) -> Port8PriorityR {
        Port8PriorityR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Escalate Priority Load Value"]
    #[inline(always)]
    #[must_use]
    pub fn escalate_priority_load(&mut self) -> EscalatePriorityLoadW<CpswNcPtypeRegSpec> {
        EscalatePriorityLoadW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 0 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_priority(&mut self) -> Port0PriorityW<CpswNcPtypeRegSpec> {
        Port0PriorityW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 1 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_1_priority(&mut self) -> Port1PriorityW<CpswNcPtypeRegSpec> {
        Port1PriorityW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 2 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_2_priority(&mut self) -> Port2PriorityW<CpswNcPtypeRegSpec> {
        Port2PriorityW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 3 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_3_priority(&mut self) -> Port3PriorityW<CpswNcPtypeRegSpec> {
        Port3PriorityW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 4 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_4_priority(&mut self) -> Port4PriorityW<CpswNcPtypeRegSpec> {
        Port4PriorityW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 5 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_5_priority(&mut self) -> Port5PriorityW<CpswNcPtypeRegSpec> {
        Port5PriorityW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 6 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_6_priority(&mut self) -> Port6PriorityW<CpswNcPtypeRegSpec> {
        Port6PriorityW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 7 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_7_priority(&mut self) -> Port7PriorityW<CpswNcPtypeRegSpec> {
        Port7PriorityW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Port 8 Priority Type Escalate"]
    #[inline(always)]
    #[must_use]
    pub fn port_8_priority(&mut self) -> Port8PriorityW<CpswNcPtypeRegSpec> {
        Port8PriorityW::new(self, 16)
    }
}
#[doc = "CPSW Transmit Priority Type\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_ptype_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_ptype_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcPtypeRegSpec;
impl crate::RegisterSpec for CpswNcPtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_ptype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcPtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_ptype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcPtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_PTYPE_REG to value 0"]
impl crate::Resettable for CpswNcPtypeRegSpec {
    const RESET_VALUE: u32 = 0;
}
