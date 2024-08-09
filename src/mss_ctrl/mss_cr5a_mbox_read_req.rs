#[doc = "Register `MSS_CR5A_MBOX_READ_REQ` reader"]
pub type R = crate::R<MssCr5aMboxReadReqSpec>;
#[doc = "Register `MSS_CR5A_MBOX_READ_REQ` writer"]
pub type W = crate::W<MssCr5aMboxReadReqSpec>;
#[doc = "Field `proc_0` reader - 0:0\\]
This is request from processor 0 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc0R = crate::BitReader;
#[doc = "Field `proc_0` writer - 0:0\\]
This is request from processor 0 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_1` reader - 4:4\\]
This is request from processor 1 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc1R = crate::BitReader;
#[doc = "Field `proc_1` writer - 4:4\\]
This is request from processor 1 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_2` reader - 8:8\\]
This is request from processor 2 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc2R = crate::BitReader;
#[doc = "Field `proc_2` writer - 8:8\\]
This is request from processor 2 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_3` reader - 12:12\\]
This is request from processor 3 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc3R = crate::BitReader;
#[doc = "Field `proc_3` writer - 12:12\\]
This is request from processor 3 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_4` reader - 16:16\\]
This is request from processor 4 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc4R = crate::BitReader;
#[doc = "Field `proc_4` writer - 16:16\\]
This is request from processor 4 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_5` reader - 20:20\\]
This is request from processor 5 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc5R = crate::BitReader;
#[doc = "Field `proc_5` writer - 20:20\\]
This is request from processor 5 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_6` reader - 24:24\\]
This is request from processor 6 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc6R = crate::BitReader;
#[doc = "Field `proc_6` writer - 24:24\\]
This is request from processor 6 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc_7` reader - 28:28\\]
This is request from processor 7 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc7R = crate::BitReader;
#[doc = "Field `proc_7` writer - 28:28\\]
This is request from processor 7 to mss_cr5a. Requesting it to read from mailbox."]
pub type Proc7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is request from processor 0 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_0(&self) -> Proc0R {
        Proc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This is request from processor 1 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_1(&self) -> Proc1R {
        Proc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This is request from processor 2 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_2(&self) -> Proc2R {
        Proc2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This is request from processor 3 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_3(&self) -> Proc3R {
        Proc3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This is request from processor 4 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_4(&self) -> Proc4R {
        Proc4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
This is request from processor 5 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_5(&self) -> Proc5R {
        Proc5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
This is request from processor 6 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_6(&self) -> Proc6R {
        Proc6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
This is request from processor 7 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    pub fn proc_7(&self) -> Proc7R {
        Proc7R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This is request from processor 0 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_0(&mut self) -> Proc0W<MssCr5aMboxReadReqSpec> {
        Proc0W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This is request from processor 1 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_1(&mut self) -> Proc1W<MssCr5aMboxReadReqSpec> {
        Proc1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
This is request from processor 2 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_2(&mut self) -> Proc2W<MssCr5aMboxReadReqSpec> {
        Proc2W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
This is request from processor 3 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_3(&mut self) -> Proc3W<MssCr5aMboxReadReqSpec> {
        Proc3W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
This is request from processor 4 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_4(&mut self) -> Proc4W<MssCr5aMboxReadReqSpec> {
        Proc4W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
This is request from processor 5 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_5(&mut self) -> Proc5W<MssCr5aMboxReadReqSpec> {
        Proc5W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
This is request from processor 6 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_6(&mut self) -> Proc6W<MssCr5aMboxReadReqSpec> {
        Proc6W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
This is request from processor 7 to mss_cr5a. Requesting it to read from mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn proc_7(&mut self) -> Proc7W<MssCr5aMboxReadReqSpec> {
        Proc7W::new(self, 28)
    }
}
#[doc = "MSS_CR5A_MBOX_READ_REQ\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cr5a_mbox_read_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cr5a_mbox_read_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCr5aMboxReadReqSpec;
impl crate::RegisterSpec for MssCr5aMboxReadReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_cr5a_mbox_read_req::R`](R) reader structure"]
impl crate::Readable for MssCr5aMboxReadReqSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_cr5a_mbox_read_req::W`](W) writer structure"]
impl crate::Writable for MssCr5aMboxReadReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CR5A_MBOX_READ_REQ to value 0"]
impl crate::Resettable for MssCr5aMboxReadReqSpec {
    const RESET_VALUE: u32 = 0;
}
