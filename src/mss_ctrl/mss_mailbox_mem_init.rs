#[doc = "Register `MSS_MAILBOX_MEM_INIT` reader"]
pub type R = crate::R<MssMailboxMemInitSpec>;
#[doc = "Register `MSS_MAILBOX_MEM_INIT` writer"]
pub type W = crate::W<MssMailboxMemInitSpec>;
#[doc = "Field `mem0_init` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_MBOX. Value in each row is initialized to 0x0"]
pub type Mem0InitR = crate::BitReader;
#[doc = "Field `mem0_init` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_MBOX. Value in each row is initialized to 0x0"]
pub type Mem0InitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_MBOX. Value in each row is initialized to 0x0"]
    #[inline(always)]
    pub fn mem0_init(&self) -> Mem0InitR {
        Mem0InitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_MBOX. Value in each row is initialized to 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_init(&mut self) -> Mem0InitW<MssMailboxMemInitSpec> {
        Mem0InitW::new(self, 0)
    }
}
#[doc = "MSS_MAILBOX_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_mailbox_mem_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_mailbox_mem_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssMailboxMemInitSpec;
impl crate::RegisterSpec for MssMailboxMemInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_mailbox_mem_init::R`](R) reader structure"]
impl crate::Readable for MssMailboxMemInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_mailbox_mem_init::W`](W) writer structure"]
impl crate::Writable for MssMailboxMemInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_MAILBOX_MEM_INIT to value 0"]
impl crate::Resettable for MssMailboxMemInitSpec {
    const RESET_VALUE: u32 = 0;
}
