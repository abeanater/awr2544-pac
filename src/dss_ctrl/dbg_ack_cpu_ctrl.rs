#[doc = "Register `DBG_ACK_CPU_CTRL` reader"]
pub type R = crate::R<DbgAckCpuCtrlSpec>;
#[doc = "Register `DBG_ACK_CPU_CTRL` writer"]
pub type W = crate::W<DbgAckCpuCtrlSpec>;
#[doc = "Field `sel` reader - 0:0\\]
Select the Processor Suspend that is used to Suspend the DSS Peripehrals 0: DSP 1:MSS CR5"]
pub type SelR = crate::BitReader;
#[doc = "Field `sel` writer - 0:0\\]
Select the Processor Suspend that is used to Suspend the DSS Peripehrals 0: DSP 1:MSS CR5"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select the Processor Suspend that is used to Suspend the DSS Peripehrals 0: DSP 1:MSS CR5"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select the Processor Suspend that is used to Suspend the DSS Peripehrals 0: DSP 1:MSS CR5"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DbgAckCpuCtrlSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "DBG_ACK_CPU_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ack_cpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ack_cpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgAckCpuCtrlSpec;
impl crate::RegisterSpec for DbgAckCpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_ack_cpu_ctrl::R`](R) reader structure"]
impl crate::Readable for DbgAckCpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_ack_cpu_ctrl::W`](W) writer structure"]
impl crate::Writable for DbgAckCpuCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_ACK_CPU_CTRL to value 0"]
impl crate::Resettable for DbgAckCpuCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
