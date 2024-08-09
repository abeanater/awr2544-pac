#[doc = "Register `MSS_CR5A_MBOX_READ_DONE` reader"]
pub type R = crate::R<MssCr5aMboxReadDoneSpec>;
#[doc = "Register `MSS_CR5A_MBOX_READ_DONE` writer"]
pub type W = crate::W<MssCr5aMboxReadDoneSpec>;
#[doc = "Field `proc_0` reader - 0:0\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 0"]
pub type Proc0R = crate::BitReader;
#[doc = "Field `proc_0` writer - 0:0\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 0"]
pub type Proc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_1` reader - 4:4\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 1"]
pub type Proc1R = crate::BitReader;
#[doc = "Field `proc_1` writer - 4:4\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 1"]
pub type Proc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_2` reader - 8:8\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 2"]
pub type Proc2R = crate::BitReader;
#[doc = "Field `proc_2` writer - 8:8\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 2"]
pub type Proc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_3` reader - 12:12\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 3"]
pub type Proc3R = crate::BitReader;
#[doc = "Field `proc_3` writer - 12:12\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 3"]
pub type Proc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_4` reader - 16:16\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 4"]
pub type Proc4R = crate::BitReader;
#[doc = "Field `proc_4` writer - 16:16\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 4"]
pub type Proc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_5` reader - 20:20\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 5"]
pub type Proc5R = crate::BitReader;
#[doc = "Field `proc_5` writer - 20:20\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 5"]
pub type Proc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_6` reader - 24:24\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 6"]
pub type Proc6R = crate::BitReader;
#[doc = "Field `proc_6` writer - 24:24\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 6"]
pub type Proc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_7` reader - 28:28\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 7"]
pub type Proc7R = crate::BitReader;
#[doc = "Field `proc_7` writer - 28:28\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 7"]
pub type Proc7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 0"]
    #[inline(always)]
    pub fn proc_0(&self) -> Proc0R {
        Proc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 1"]
    #[inline(always)]
    pub fn proc_1(&self) -> Proc1R {
        Proc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 2"]
    #[inline(always)]
    pub fn proc_2(&self) -> Proc2R {
        Proc2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 3"]
    #[inline(always)]
    pub fn proc_3(&self) -> Proc3R {
        Proc3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 4"]
    #[inline(always)]
    pub fn proc_4(&self) -> Proc4R {
        Proc4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 5"]
    #[inline(always)]
    pub fn proc_5(&self) -> Proc5R {
        Proc5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 6"]
    #[inline(always)]
    pub fn proc_6(&self) -> Proc6R {
        Proc6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 7"]
    #[inline(always)]
    pub fn proc_7(&self) -> Proc7R {
        Proc7R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 0"]
    #[inline(always)]
    #[must_use]
    pub fn proc_0(&mut self) -> Proc0W<MssCr5aMboxReadDoneSpec> {
        Proc0W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 1"]
    #[inline(always)]
    #[must_use]
    pub fn proc_1(&mut self) -> Proc1W<MssCr5aMboxReadDoneSpec> {
        Proc1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 2"]
    #[inline(always)]
    #[must_use]
    pub fn proc_2(&mut self) -> Proc2W<MssCr5aMboxReadDoneSpec> {
        Proc2W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 3"]
    #[inline(always)]
    #[must_use]
    pub fn proc_3(&mut self) -> Proc3W<MssCr5aMboxReadDoneSpec> {
        Proc3W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 4"]
    #[inline(always)]
    #[must_use]
    pub fn proc_4(&mut self) -> Proc4W<MssCr5aMboxReadDoneSpec> {
        Proc4W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 5"]
    #[inline(always)]
    #[must_use]
    pub fn proc_5(&mut self) -> Proc5W<MssCr5aMboxReadDoneSpec> {
        Proc5W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 6"]
    #[inline(always)]
    #[must_use]
    pub fn proc_6(&mut self) -> Proc6W<MssCr5aMboxReadDoneSpec> {
        Proc6W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
This register should be written once finishing reading from CR5A's mailbox written by proc 7"]
    #[inline(always)]
    #[must_use]
    pub fn proc_7(&mut self) -> Proc7W<MssCr5aMboxReadDoneSpec> {
        Proc7W::new(self, 28)
    }
}
#[doc = "MSS_CR5A_MBOX_READ_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_mbox_read_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_mbox_read_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCr5aMboxReadDoneSpec;
impl crate::RegisterSpec for MssCr5aMboxReadDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_cr5a_mbox_read_done::R`](R) reader structure"]
impl crate::Readable for MssCr5aMboxReadDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_cr5a_mbox_read_done::W`](W) writer structure"]
impl crate::Writable for MssCr5aMboxReadDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CR5A_MBOX_READ_DONE to value 0"]
impl crate::Resettable for MssCr5aMboxReadDoneSpec {
    const RESET_VALUE: u32 = 0;
}
