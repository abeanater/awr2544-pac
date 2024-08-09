#[doc = "Register `NW_PACKET_COUNT` reader"]
pub type R = crate::R<NwPacketCountSpec>;
#[doc = "Register `NW_PACKET_COUNT` writer"]
pub type W = crate::W<NwPacketCountSpec>;
#[doc = "Field `stat` reader - 31:0\\]
Gives the count of the ping-pong switch events of the network packet buffer"]
pub type StatR = crate::FieldReader<u32>;
#[doc = "Field `stat` writer - 31:0\\]
Gives the count of the ping-pong switch events of the network packet buffer"]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Gives the count of the ping-pong switch events of the network packet buffer"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Gives the count of the ping-pong switch events of the network packet buffer"]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<NwPacketCountSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "NW_PACKET_COUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`nw_packet_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nw_packet_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NwPacketCountSpec;
impl crate::RegisterSpec for NwPacketCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nw_packet_count::R`](R) reader structure"]
impl crate::Readable for NwPacketCountSpec {}
#[doc = "`write(|w| ..)` method takes [`nw_packet_count::W`](W) writer structure"]
impl crate::Writable for NwPacketCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NW_PACKET_COUNT to value 0"]
impl crate::Resettable for NwPacketCountSpec {
    const RESET_VALUE: u32 = 0;
}
