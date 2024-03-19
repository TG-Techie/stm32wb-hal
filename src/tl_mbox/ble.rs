use crate::ipcc::Ipcc;
use crate::tl_mbox::channels;
use crate::tl_mbox::cmd::{CmdPacket, CmdSerial};
use crate::tl_mbox::consts::TlPacketType;
use crate::tl_mbox::evt::EvtBox;
use crate::tl_mbox::unsafe_linked_list::{
    LST_init_head, LST_is_empty, LST_remove_head, LinkedListNode,
};
use crate::tl_mbox::{
    evt, BleTable, HeaplessEvtQueue, BLE_CMD_BUFFER, CS_BUFFER, EVT_QUEUE, HCI_ACL_DATA_BUFFER,
    TL_BLE_TABLE, TL_REF_TABLE,
};
use core::mem::MaybeUninit;

pub struct Ble {}

impl Ble {
    pub(super) fn new(ipcc: &mut Ipcc) -> Self {
        unsafe {
            LST_init_head(EVT_QUEUE.as_mut_ptr());

            TL_BLE_TABLE = MaybeUninit::new(BleTable {
                pcmd_buffer: BLE_CMD_BUFFER.as_mut_ptr().cast(),
                pcs_buffer: CS_BUFFER.as_ptr().cast(),
                pevt_queue: EVT_QUEUE.as_ptr().cast(),
                phci_acl_data_buffer: HCI_ACL_DATA_BUFFER.as_mut_ptr().cast(),
            });
        }

        ipcc.c1_set_rx_channel(channels::cpu2::IPCC_BLE_EVENT_CHANNEL, true);

        Ble {}
    }

    pub(super) fn evt_handler(&self, ipcc: &mut Ipcc, queue: &mut HeaplessEvtQueue) {
        unsafe {
            let mut node_ptr: *mut LinkedListNode = core::ptr::null_mut();
            let node_ptr_ptr: *mut *mut LinkedListNode = &mut node_ptr;

            while !LST_is_empty(EVT_QUEUE.as_mut_ptr()) {
                LST_remove_head(EVT_QUEUE.as_mut_ptr(), node_ptr_ptr);

                let event: *mut evt::EvtPacket = node_ptr.cast();
                let event = EvtBox::new(event);

                queue.enqueue(event).unwrap();
            }
        }

        ipcc.c1_clear_flag_channel(channels::cpu2::IPCC_BLE_EVENT_CHANNEL);
    }

    pub(super) fn acl_data_handler(&self, ipcc: &mut Ipcc) {
        ipcc.c1_set_tx_channel(channels::cpu1::IPCC_HCI_ACL_DATA_CHANNEL, false);

        // TODO: ACL data ack to the user
    }
}

pub fn ble_send_cmd(ipcc: &mut Ipcc, buf: &[u8]) {
    // TG-TODO-MAYBE: narrow unsafe block
    unsafe {
        let pcmd_buffer: *mut CmdPacket = (&*TL_REF_TABLE.assume_init().ble_table).pcmd_buffer;
        let pcmd_serial: *mut CmdSerial = &mut (*pcmd_buffer).cmdserial;
        let pcmd_serial_buf: *mut u8 = pcmd_serial.cast();

        core::ptr::copy(buf.as_ptr(), pcmd_serial_buf, buf.len());

        // TG-CHANGE: mut removed from cmd_packet
        let cmd_packet = &mut *(&*TL_REF_TABLE.assume_init().ble_table).pcmd_buffer;
        cmd_packet.cmdserial.ty = TlPacketType::BleCmd as u8;
    }

    ipcc.c1_set_flag_channel(channels::cpu1::IPCC_BLE_CMD_CHANNEL);
}

#[allow(dead_code)] // Not used currently but reserved
pub(super) fn ble_send_acl_data(ipcc: &mut Ipcc) {
    // TG-CHANGE: remove mut from cmd_packet
    let cmd_packet = unsafe { &mut *(*TL_REF_TABLE.assume_init().ble_table).phci_acl_data_buffer };
    cmd_packet.acl_data_serial.ty = TlPacketType::AclData as u8;

    ipcc.c1_set_flag_channel(channels::cpu1::IPCC_HCI_ACL_DATA_CHANNEL);
    ipcc.c1_set_tx_channel(channels::cpu1::IPCC_HCI_ACL_DATA_CHANNEL, true);
}
