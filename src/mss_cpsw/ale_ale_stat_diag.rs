#[doc = "Register `ALE_ALE_STAT_DIAG` reader"]
pub type R = crate::R<AleAleStatDiagSpec>;
#[doc = "Register `ALE_ALE_STAT_DIAG` writer"]
pub type W = crate::W<AleAleStatDiagSpec>;
#[doc = "Field `WHEN_NONZERO_WILL` reader - 3:0\\]
When non-zero will cause the selected statistic to increment on the next frame received. For the selected Port. #br# 0: Disabled#br# 1: Destination Equal Source Drop Stat will count#br# 2: VLAN Ingress Check Drop Stat will count#br# 3: Source Multicast Drop Stat will count#br# 4: Dual VLAN Drop Stat will count#br# 5: Ether Type length error Drop Stat will count#br# 6: Next Hop Limit Drop Stat will count#br# 7: IPv4 Fragment Drop Stat will count#br# 8: Classifier Hit Stat will count#br# 9: Classifier Red Drop Stat will count#br# 10: Classifier Yellow Drop Stat will count#br# 11: ALE Overflow Drop Stat will count#br# 12: Rate Limit Drop Stat will count#br# 13: Blocked Address Drop Stat will count#br# 14: Secure Address Drop Stat will count#br# 15: Authorization Drop Stat will count."]
pub type WhenNonzeroWillR = crate::FieldReader;
#[doc = "Field `WHEN_NONZERO_WILL` writer - 3:0\\]
When non-zero will cause the selected statistic to increment on the next frame received. For the selected Port. #br# 0: Disabled#br# 1: Destination Equal Source Drop Stat will count#br# 2: VLAN Ingress Check Drop Stat will count#br# 3: Source Multicast Drop Stat will count#br# 4: Dual VLAN Drop Stat will count#br# 5: Ether Type length error Drop Stat will count#br# 6: Next Hop Limit Drop Stat will count#br# 7: IPv4 Fragment Drop Stat will count#br# 8: Classifier Hit Stat will count#br# 9: Classifier Red Drop Stat will count#br# 10: Classifier Yellow Drop Stat will count#br# 11: ALE Overflow Drop Stat will count#br# 12: Rate Limit Drop Stat will count#br# 13: Blocked Address Drop Stat will count#br# 14: Secure Address Drop Stat will count#br# 15: Authorization Drop Stat will count."]
pub type WhenNonzeroWillW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THE_PORT_SELECTED` reader - 8:8\\]
The port selected that a received packet will cause the selected error to increment"]
pub type ThePortSelectedR = crate::BitReader;
#[doc = "Field `THE_PORT_SELECTED` writer - 8:8\\]
The port selected that a received packet will cause the selected error to increment"]
pub type ThePortSelectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHEN_SET_AND` reader - 15:15\\]
When set and the ~iport_diag is set to zero, will allow all ports to see the same stat diagnostic increment."]
pub type WhenSetAndR = crate::BitReader;
#[doc = "Field `WHEN_SET_AND` writer - 15:15\\]
When set and the ~iport_diag is set to zero, will allow all ports to see the same stat diagnostic increment."]
pub type WhenSetAndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
When non-zero will cause the selected statistic to increment on the next frame received. For the selected Port. #br# 0: Disabled#br# 1: Destination Equal Source Drop Stat will count#br# 2: VLAN Ingress Check Drop Stat will count#br# 3: Source Multicast Drop Stat will count#br# 4: Dual VLAN Drop Stat will count#br# 5: Ether Type length error Drop Stat will count#br# 6: Next Hop Limit Drop Stat will count#br# 7: IPv4 Fragment Drop Stat will count#br# 8: Classifier Hit Stat will count#br# 9: Classifier Red Drop Stat will count#br# 10: Classifier Yellow Drop Stat will count#br# 11: ALE Overflow Drop Stat will count#br# 12: Rate Limit Drop Stat will count#br# 13: Blocked Address Drop Stat will count#br# 14: Secure Address Drop Stat will count#br# 15: Authorization Drop Stat will count."]
    #[inline(always)]
    pub fn when_nonzero_will(&self) -> WhenNonzeroWillR {
        WhenNonzeroWillR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
The port selected that a received packet will cause the selected error to increment"]
    #[inline(always)]
    pub fn the_port_selected(&self) -> ThePortSelectedR {
        ThePortSelectedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
When set and the ~iport_diag is set to zero, will allow all ports to see the same stat diagnostic increment."]
    #[inline(always)]
    pub fn when_set_and(&self) -> WhenSetAndR {
        WhenSetAndR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
When non-zero will cause the selected statistic to increment on the next frame received. For the selected Port. #br# 0: Disabled#br# 1: Destination Equal Source Drop Stat will count#br# 2: VLAN Ingress Check Drop Stat will count#br# 3: Source Multicast Drop Stat will count#br# 4: Dual VLAN Drop Stat will count#br# 5: Ether Type length error Drop Stat will count#br# 6: Next Hop Limit Drop Stat will count#br# 7: IPv4 Fragment Drop Stat will count#br# 8: Classifier Hit Stat will count#br# 9: Classifier Red Drop Stat will count#br# 10: Classifier Yellow Drop Stat will count#br# 11: ALE Overflow Drop Stat will count#br# 12: Rate Limit Drop Stat will count#br# 13: Blocked Address Drop Stat will count#br# 14: Secure Address Drop Stat will count#br# 15: Authorization Drop Stat will count."]
    #[inline(always)]
    #[must_use]
    pub fn when_nonzero_will(&mut self) -> WhenNonzeroWillW<AleAleStatDiagSpec> {
        WhenNonzeroWillW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
The port selected that a received packet will cause the selected error to increment"]
    #[inline(always)]
    #[must_use]
    pub fn the_port_selected(&mut self) -> ThePortSelectedW<AleAleStatDiagSpec> {
        ThePortSelectedW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
When set and the ~iport_diag is set to zero, will allow all ports to see the same stat diagnostic increment."]
    #[inline(always)]
    #[must_use]
    pub fn when_set_and(&mut self) -> WhenSetAndW<AleAleStatDiagSpec> {
        WhenSetAndW::new(self, 15)
    }
}
#[doc = "The ALE Statistic Output Diagnostic Register allows the output statistics to diagnose the SW counters. This register is for diagnostice only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_stat_diag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_stat_diag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleStatDiagSpec;
impl crate::RegisterSpec for AleAleStatDiagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_stat_diag::R`](R) reader structure"]
impl crate::Readable for AleAleStatDiagSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_stat_diag::W`](W) writer structure"]
impl crate::Writable for AleAleStatDiagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_STAT_DIAG to value 0"]
impl crate::Resettable for AleAleStatDiagSpec {
    const RESET_VALUE: u32 = 0;
}
