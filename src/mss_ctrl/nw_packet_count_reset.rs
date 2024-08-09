#[doc = "Register `NW_PACKET_COUNT_RESET` reader"]
pub type R = crate::R<NwPacketCountResetSpec>;
#[doc = "Register `NW_PACKET_COUNT_RESET` writer"]
pub type W = crate::W<NwPacketCountResetSpec>;
#[doc = "Field `undefined` reader - 0:0\\]
Software reset for Counter maintaining Ping pong switch events"]
pub type UndefinedR = crate::BitReader;
#[doc = "Field `undefined` writer - 0:0\\]
Software reset for Counter maintaining Ping pong switch events"]
pub type UndefinedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset for Counter maintaining Ping pong switch events"]
    #[inline(always)]
    pub fn undefined(&self) -> UndefinedR {
        UndefinedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset for Counter maintaining Ping pong switch events"]
    #[inline(always)]
    #[must_use]
    pub fn undefined(&mut self) -> UndefinedW<NwPacketCountResetSpec> {
        UndefinedW::new(self, 0)
    }
}
#[doc = "NW_PACKET_COUNT_RESET\n\nYou can [`read`](crate::Reg::read) this register and get [`nw_packet_count_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nw_packet_count_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NwPacketCountResetSpec;
impl crate::RegisterSpec for NwPacketCountResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nw_packet_count_reset::R`](R) reader structure"]
impl crate::Readable for NwPacketCountResetSpec {}
#[doc = "`write(|w| ..)` method takes [`nw_packet_count_reset::W`](W) writer structure"]
impl crate::Writable for NwPacketCountResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NW_PACKET_COUNT_RESET to value 0"]
impl crate::Resettable for NwPacketCountResetSpec {
    const RESET_VALUE: u32 = 0;
}
