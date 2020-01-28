use lazy_static::lazy_static;

use super::*;

pub const SERVER_DEMAND_ACTIVE_BUFFER: [u8; 357] = [
    0x04, 0x00, // source descriptor length
    0x59, 0x01, // combined length
    0x52, 0x44, 0x50, 0x00, // source descriptor
    0x0d, 0x00, // capabilities count
    0x00, 0x00, // padding
    0x09, 0x00, 0x08, 0x00, 0xea, 0x03, 0xdc, 0xe2, // Share capability set,
    0x01, 0x00, 0x18, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x04,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, // General capability set
    0x14, 0x00, 0x0c, 0x00, 0x02, 0x00, 0x00, 0x00, 0x40, 0x06, 0x00,
    0x00, // VirtualChannel capability set
    0x16, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0xf6, 0x13, 0xf3, 0x01, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x9c, 0xf6, 0x13, 0xf3, 0x61, 0xa6, 0x82, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x91, 0xbf, // DrawGdiPlus capability set
    0x0e, 0x00, 0x08, 0x00, 0x00, 0x01, 0x00, 0x00, // Font capability set
    0x02, 0x00, 0x1c, 0x00, 0x18, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x04,
    0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, // Bitmap capability set
    0x03, 0x00, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x00, 0x00, 0x22, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x01, 0x01, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42, 0x0f, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Order capability set
    0x0a, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, // ColorCache capability set
    0x12, 0x00, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00, // BitmapCache capability set
    0x08, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x19, 0x00, 0x19, 0x00, // Pointer capability set
    0x0d, 0x00, 0x58, 0x00, 0x35, 0x00, 0x00, 0x00, 0xa1, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0c, 0xf6, 0x13, 0xf3, 0x93, 0x5a, 0x37, 0xf3, 0x00, 0x90, 0x30, 0xe1, 0x34, 0x1c, 0x38, 0xf3,
    0x40, 0xf6, 0x13, 0xf3, 0x04, 0x00, 0x00, 0x00, 0x4c, 0x54, 0xdc, 0xe2, 0x08, 0x50, 0xdc, 0xe2,
    0x01, 0x00, 0x00, 0x00, 0x08, 0x50, 0xdc, 0xe2, 0x00, 0x00, 0x00, 0x00, 0x38, 0xf6, 0x13, 0xf3,
    0x2e, 0x05, 0x38, 0xf3, 0x08, 0x50, 0xdc, 0xe2, 0x2c, 0xf6, 0x13, 0xf3, 0x00, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00, 0x00, // Input capability set
    0x17, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // Rail capability set
    0x18, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // WindowList capability set
    0x00, 0x00, 0x00, 0x00, // session id
];
pub const CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET_BUFFER: [u8; 501] = [
    0xea, 0x3, // originator ID
    0x6, 0x0, // source descriptor length
    0xe9, 0x1, // combined length
    0x4d, 0x53, 0x54, 0x53, 0x43, 0x0, // source descriptor
    0x14, 0x0, // capability sets count
    0x0, 0x0, // padding
    0x1, 0x0, 0x18, 0x0, 0x1, 0x0, 0x3, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x1d, 0x4, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // general capability set
    0x2, 0x0, 0x1c, 0x0, 0x20, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x20, 0x3, 0x58, 0x2, 0x0, 0x0,
    0x1, 0x0, 0x1, 0x0, 0x0, 0xa, 0x1, 0x0, 0x0, 0x0, // bitmap
    0x3, 0x0, 0x58, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x14, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2a, 0x0, 0x1,
    0x1, 0x1, 0x1, 0x1, 0x0, 0x0, 0x1, 0x1, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1,
    0x1, 0x1, 0x1, 0x0, 0x1, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x84, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0xe4, 0x4, 0x0, 0x0, // order
    0x4, 0x0, 0x28, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x78, 0x0, 0x0, 0x4, 0x78, 0x0, 0x0, 0x10, 0x51,
    0x1, 0x0, 0x40, // bitmap cache
    0xa, 0x0, 0x8, 0x0, 0x6, 0x0, 0x0, 0x0, // color cache
    0x7, 0x0, 0xc, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // window activation
    0x5, 0x0, 0xc, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x2, 0x0, // control
    0x8, 0x0, 0xa, 0x0, 0x1, 0x0, 0x14, 0x0, 0x15, 0x0, // pointer
    0x9, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, // share
    0xd, 0x0, 0x58, 0x0, 0x91, 0x0, 0x0, 0x0, 0x9, 0x4, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0xc, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // input
    0xc, 0x0, 0x8, 0x0, 0x1, 0x0, 0x0, 0x0, // sound
    0xe, 0x0, 0x8, 0x0, 0x1, 0x0, 0x0, 0x0, // font
    0x10, 0x0, 0x34, 0x0, 0xfe, 0x0, 0x4, 0x0, 0xfe, 0x0, 0x4, 0x0, 0xfe, 0x0, 0x8, 0x0, 0xfe, 0x0,
    0x8, 0x0, 0xfe, 0x0, 0x10, 0x0, 0xfe, 0x0, 0x20, 0x0, 0xfe, 0x0, 0x40, 0x0, 0xfe, 0x0, 0x80,
    0x0, 0xfe, 0x0, 0x0, 0x1, 0x40, 0x0, 0x0, 0x8, 0x0, 0x1, 0x0, 0x1, 0x3, 0x0, 0x0,
    0x0, // glyph cache
    0xf, 0x0, 0x8, 0x0, 0x1, 0x0, 0x0, 0x0, // brush
    0x11, 0x0, 0xc, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1e, 0x64, 0x0, // offscreen bitmap cache
    0x14, 0x0, 0x8, 0x0, 0x1, 0x0, 0x0, 0x0, // virtual channel
    0x15, 0x0, 0xc, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0xa, 0x0, 0x1, // draw nine grid cache
    0x16, 0x0, 0x28, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, // draw GDI plus
    0x1a, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, // multi fragment update
    0x18, 0x0, 0xb, 0x0, 0x1, 0x0, 0x0, 0x0, 0x3, 0xc, 0x0, // window list
];
pub const CLIENT_DEMAND_ACTIVE_BUFFER: [u8; 486] = [
    0xea, 0x03, // originator ID
    0x06, 0x00, // source descriptor length
    0xda, 0x01, // combined length
    0x4d, 0x53, 0x54, 0x53, 0x43, 0x00, // source descriptor
    0x12, 0x00, // capabilities count
    0x00, 0x00, // padding
    0x01, 0x00, 0x18, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x04,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // general capability set
    0x02, 0x00, 0x1c, 0x00, 0x18, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x04,
    0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, // bitmap capability set
    0x03, 0x00, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x00, 0x00, 0x2a, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x03, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // order capability set
    0x13, 0x00, 0x28, 0x00, 0x03, 0x00, 0x00, 0x03, 0x78, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00,
    0xfb, 0x09, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bitmap cache rev 2 capability set
    0x0a, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, // color cache
    0x07, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // window activation capability set
    0x05, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02,
    0x00, // control capability set
    0x08, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x14, 0x00, 0x15, 0x00, // pointer capability set
    0x09, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // share capability set
    0x0d, 0x00, 0x58, 0x00, 0x15, 0x00, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // input capability set
    0x0c, 0x00, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00, // sound capability set
    0x0e, 0x00, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00, // font capability set
    0x10, 0x00, 0x34, 0x00, 0xfe, 0x00, 0x04, 0x00, 0xfe, 0x00, 0x04, 0x00, 0xfe, 0x00, 0x08, 0x00,
    0xfe, 0x00, 0x08, 0x00, 0xfe, 0x00, 0x10, 0x00, 0xfe, 0x00, 0x20, 0x00, 0xfe, 0x00, 0x40, 0x00,
    0xfe, 0x00, 0x80, 0x00, 0xfe, 0x00, 0x00, 0x01, 0x40, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00, 0x01,
    0x03, 0x00, 0x00, 0x00, // glyph cache capability set
    0x0f, 0x00, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00, // brush capability set
    0x11, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x64,
    0x00, // offscreen bitmap cache capability set
    0x14, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x40, 0x06, 0x00,
    0x00, // virtual channel capability set
    0x15, 0x00, 0x0c, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00,
    0x01, // draw nine grid cache capability set
    0x16, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // draw gdi plus capability set
];
const SERVER_SHARE_CAPABILITY_SET: [u8; 4] = [0xea, 0x03, 0xdc, 0xe2];
const SERVER_GENERAL_CAPABILITY_SET: [u8; 20] = [
    0x01, 0x00, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x04, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x01,
];
const SERVER_VIRTUAL_CHANNEL_CAPABILITY_SET: [u8; 8] =
    [0x02, 0x00, 0x00, 0x00, 0x40, 0x06, 0x00, 0x00];
const SERVER_DRAW_GDI_PLUS_CAPABILITY_SET: [u8; 36] = [
    0x00, 0x00, 0x00, 0x00, 0x70, 0xf6, 0x13, 0xf3, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x18, 0x00, 0x00, 0x00, 0x9c, 0xf6, 0x13, 0xf3, 0x61, 0xa6, 0x82, 0x80, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x50, 0x91, 0xbf,
];
const SERVER_FONT_CAPABILITY_SET: [u8; 4] = [0x00, 0x01, 0x00, 0x00];
const SERVER_BITMAP_CAPABILITY_SET: [u8; 24] = [
    0x18, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00, 0x00, 0x01, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
];
const SERVER_ORDER_CAPABILITY_SET: [u8; 84] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const SERVER_COLOR_CACHE_CAPABILITY_SET: [u8; 4] = [0x06, 0x00, 0x00, 0x00];
const SERVER_BITMAP_CACHE_HOST_SUPPORT_CAPABILITY_SET: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const SERVER_POINTER_CAPABILITY_SET: [u8; 6] = [0x01, 0x00, 0x19, 0x00, 0x19, 0x00];
const SERVER_INPUT_CAPABILITY_SET: [u8; 84] = [
    0x35, 0x00, 0x00, 0x00, 0xa1, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0xf6, 0x13, 0xf3,
    0x93, 0x5a, 0x37, 0xf3, 0x00, 0x90, 0x30, 0xe1, 0x34, 0x1c, 0x38, 0xf3, 0x40, 0xf6, 0x13, 0xf3,
    0x04, 0x00, 0x00, 0x00, 0x4c, 0x54, 0xdc, 0xe2, 0x08, 0x50, 0xdc, 0xe2, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x50, 0xdc, 0xe2, 0x00, 0x00, 0x00, 0x00, 0x38, 0xf6, 0x13, 0xf3, 0x2e, 0x05, 0x38, 0xf3,
    0x08, 0x50, 0xdc, 0xe2, 0x2c, 0xf6, 0x13, 0xf3, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x0a, 0x00,
    0x01, 0x00, 0x00, 0x00,
];
const SERVER_RAIL_CAPABILITY_SET: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
const SERVER_WINDOW_LIST_CAPABILITY_SET: [u8; 7] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

const CLIENT_GENERAL_CAPABILITY_SET: [u8; 20] = [
    0x01, 0x00, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x04, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const CLIENT_BITMAP_CAPABILITY_SET: [u8; 24] = [
    0x18, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00, 0x00, 0x01, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
];
const CLIENT_BITMAP_CAPABILITY_SET_32_BIT: [u8; 24] = [
    0x20, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x20, 0x3, 0x58, 0x2, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0,
    0x0, 0xa, 0x1, 0x0, 0x0, 0x0,
];
const CLIENT_ORDER_CAPABILITY_SET: [u8; 84] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x2a, 0x00,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const CLIENT_ORDER_CAPABILITY_SET_ANSI_CODE_PAGE: [u8; 84] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x1, 0x0, 0x14, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2a, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1,
    0x0, 0x0, 0x1, 0x1, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x0,
    0x1, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x84, 0x3,
    0x0, 0x0, 0x0, 0x0, 0x0, 0xe4, 0x4, 0x0, 0x0,
];
const CLIENT_BITMAP_CACHE_REV_2_CAPABILITY_SET: [u8; 36] = [
    0x03, 0x00, 0x00, 0x03, 0x78, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0xfb, 0x09, 0x00, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const CLIENT_BITMAP_CACHE_REV_1_CAPABILITY_SET: [u8; 36] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x78, 0x0, 0x0, 0x4, 0x78, 0x0, 0x0, 0x10, 0x51, 0x1, 0x0, 0x40,
];
const CLIENT_COLOR_CACHE_CAPABILITY_SET: [u8; 4] = [0x06, 0x00, 0x00, 0x00];
const CLIENT_WINDOW_ACTIVATION_CAPABILITY_SET: [u8; 8] =
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const CLIENT_CONTROL_CAPABILITY_SET: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00];
const CLIENT_POINTER_CAPABILITY_SET: [u8; 6] = [0x01, 0x00, 0x14, 0x00, 0x15, 0x00];
const CLIENT_SHARE_CAPABILITY_SET: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
const CLIENT_INPUT_CAPABILITY_SET: [u8; 84] = [
    0x15, 0x00, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const CLIENT_INPUT_CAPABILITY_SET_UNICODE: [u8; 84] = [
    0x91, 0x0, 0x20, 0x0, 0x9, 0x4, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
const CLIENT_SOUND_CAPABILITY_SET: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const CLIENT_FONT_CAPABILITY_SET: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const CLIENT_GLYPH_CACHE_CAPABILITY_SET: [u8; 48] = [
    0xfe, 0x00, 0x04, 0x00, 0xfe, 0x00, 0x04, 0x00, 0xfe, 0x00, 0x08, 0x00, 0xfe, 0x00, 0x08, 0x00,
    0xfe, 0x00, 0x10, 0x00, 0xfe, 0x00, 0x20, 0x00, 0xfe, 0x00, 0x40, 0x00, 0xfe, 0x00, 0x80, 0x00,
    0xfe, 0x00, 0x00, 0x01, 0x40, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
];
const CLIENT_BRUSH_CAPABILITY_SET: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const CLIENT_OFFSCREEN_BITMAP_CAPABILITY_SET: [u8; 8] =
    [0x01, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x64, 0x00];
const CLIENT_VIRTUAL_CHANNEL_CAPABILITY_SET_INCOMPLETE: [u8; 4] = [0x1, 0x0, 0x0, 0x0];
const CLIENT_VIRTUAL_CHANNEL_CAPABILITY_SET: [u8; 8] =
    [0x01, 0x00, 0x00, 0x00, 0x40, 0x06, 0x00, 0x00];
const CLIENT_DRAW_NINE_GRID_CACHE_CAPABILITY_SET: [u8; 8] =
    [0x02, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x01];
const CLIENT_DRAW_GDI_PLUS_CAPABILITY_SET: [u8; 36] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
const CLIENT_MULTI_FRAGMENT_UPDATE_CAPABILITY_SET: [u8; 4] = [0x0, 0x0, 0x0, 0x0];
const CLIENT_WINDOW_LIST_CAPABILITY_SET: [u8; 7] = [0x1, 0x0, 0x0, 0x0, 0x3, 0xc, 0x0];

lazy_static! {
    pub static ref SERVER_DEMAND_ACTIVE: ServerDemandActive = ServerDemandActive {
        pdu: DemandActive {
            source_descriptor: String::from("RDP"),
            capability_sets: vec![
                CapabilitySet::Share(SERVER_SHARE_CAPABILITY_SET.to_vec()),
                CapabilitySet::General(
                    General::from_buffer(SERVER_GENERAL_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::VirtualChannel(
                    VirtualChannel::from_buffer(SERVER_VIRTUAL_CHANNEL_CAPABILITY_SET.as_ref())
                        .unwrap()
                ),
                CapabilitySet::DrawGdiPlus(SERVER_DRAW_GDI_PLUS_CAPABILITY_SET.to_vec()),
                CapabilitySet::Font(SERVER_FONT_CAPABILITY_SET.to_vec()),
                CapabilitySet::Bitmap(
                    Bitmap::from_buffer(SERVER_BITMAP_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Order(
                    Order::from_buffer(SERVER_ORDER_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::ColorCache(SERVER_COLOR_CACHE_CAPABILITY_SET.to_vec()),
                CapabilitySet::BitmapCacheHostSupport(
                    SERVER_BITMAP_CACHE_HOST_SUPPORT_CAPABILITY_SET.to_vec()
                ),
                CapabilitySet::Pointer(
                    Pointer::from_buffer(SERVER_POINTER_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Input(
                    Input::from_buffer(SERVER_INPUT_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Rail(SERVER_RAIL_CAPABILITY_SET.to_vec()),
                CapabilitySet::WindowList(SERVER_WINDOW_LIST_CAPABILITY_SET.to_vec()),
            ],
        }
    };
    pub static ref CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET: ClientConfirmActive =
        ClientConfirmActive {
            originator_id: SERVER_CHANNEL_ID,
            pdu: DemandActive {
                source_descriptor: String::from("MSTSC"),
                capability_sets: vec![
                    CapabilitySet::General(
                        General::from_buffer(CLIENT_GENERAL_CAPABILITY_SET.as_ref()).unwrap()
                    ),
                    CapabilitySet::Bitmap(
                        Bitmap::from_buffer(CLIENT_BITMAP_CAPABILITY_SET_32_BIT.as_ref()).unwrap()
                    ),
                    CapabilitySet::Order(
                        Order::from_buffer(CLIENT_ORDER_CAPABILITY_SET_ANSI_CODE_PAGE.as_ref())
                            .unwrap()
                    ),
                    CapabilitySet::BitmapCache(
                        BitmapCache::from_buffer(CLIENT_BITMAP_CACHE_REV_1_CAPABILITY_SET.as_ref())
                            .unwrap()
                    ),
                    CapabilitySet::ColorCache(CLIENT_COLOR_CACHE_CAPABILITY_SET.to_vec()),
                    CapabilitySet::WindowActivation(
                        CLIENT_WINDOW_ACTIVATION_CAPABILITY_SET.to_vec()
                    ),
                    CapabilitySet::Control(CLIENT_CONTROL_CAPABILITY_SET.to_vec()),
                    CapabilitySet::Pointer(
                        Pointer::from_buffer(CLIENT_POINTER_CAPABILITY_SET.as_ref()).unwrap()
                    ),
                    CapabilitySet::Share(CLIENT_SHARE_CAPABILITY_SET.to_vec()),
                    CapabilitySet::Input(
                        Input::from_buffer(CLIENT_INPUT_CAPABILITY_SET_UNICODE.as_ref()).unwrap()
                    ),
                    CapabilitySet::Sound(
                        Sound::from_buffer(CLIENT_SOUND_CAPABILITY_SET.as_ref()).unwrap()
                    ),
                    CapabilitySet::Font(CLIENT_FONT_CAPABILITY_SET.to_vec()),
                    CapabilitySet::GlyphCache(
                        GlyphCache::from_buffer(CLIENT_GLYPH_CACHE_CAPABILITY_SET.as_ref())
                            .unwrap()
                    ),
                    CapabilitySet::Brush(
                        Brush::from_buffer(CLIENT_BRUSH_CAPABILITY_SET.as_ref()).unwrap()
                    ),
                    CapabilitySet::OffscreenBitmapCache(
                        OffscreenBitmapCache::from_buffer(
                            CLIENT_OFFSCREEN_BITMAP_CAPABILITY_SET.as_ref()
                        )
                        .unwrap()
                    ),
                    CapabilitySet::VirtualChannel(
                        VirtualChannel::from_buffer(
                            CLIENT_VIRTUAL_CHANNEL_CAPABILITY_SET_INCOMPLETE.as_ref()
                        )
                        .unwrap()
                    ),
                    CapabilitySet::DrawNineGridCache(
                        CLIENT_DRAW_NINE_GRID_CACHE_CAPABILITY_SET.to_vec()
                    ),
                    CapabilitySet::DrawGdiPlus(CLIENT_DRAW_GDI_PLUS_CAPABILITY_SET.to_vec()),
                    CapabilitySet::MultiFragmentUpdate(
                        MultifragmentUpdate::from_buffer(
                            CLIENT_MULTI_FRAGMENT_UPDATE_CAPABILITY_SET.as_ref()
                        )
                        .unwrap()
                    ),
                    CapabilitySet::WindowList(CLIENT_WINDOW_LIST_CAPABILITY_SET.to_vec()),
                ],
            }
        };
    pub static ref CLIENT_DEMAND_ACTIVE: ClientConfirmActive = ClientConfirmActive {
        originator_id: SERVER_CHANNEL_ID,
        pdu: DemandActive {
            source_descriptor: String::from("MSTSC"),
            capability_sets: vec![
                CapabilitySet::General(
                    General::from_buffer(CLIENT_GENERAL_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Bitmap(
                    Bitmap::from_buffer(CLIENT_BITMAP_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Order(
                    Order::from_buffer(CLIENT_ORDER_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::BitmapCacheRev2(
                    BitmapCacheRev2::from_buffer(CLIENT_BITMAP_CACHE_REV_2_CAPABILITY_SET.as_ref())
                        .unwrap()
                ),
                CapabilitySet::ColorCache(CLIENT_COLOR_CACHE_CAPABILITY_SET.to_vec()),
                CapabilitySet::WindowActivation(CLIENT_WINDOW_ACTIVATION_CAPABILITY_SET.to_vec()),
                CapabilitySet::Control(CLIENT_CONTROL_CAPABILITY_SET.to_vec()),
                CapabilitySet::Pointer(
                    Pointer::from_buffer(CLIENT_POINTER_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Share(CLIENT_SHARE_CAPABILITY_SET.to_vec()),
                CapabilitySet::Input(
                    Input::from_buffer(CLIENT_INPUT_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Sound(
                    Sound::from_buffer(CLIENT_SOUND_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Font(CLIENT_FONT_CAPABILITY_SET.to_vec()),
                CapabilitySet::GlyphCache(
                    GlyphCache::from_buffer(CLIENT_GLYPH_CACHE_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::Brush(
                    Brush::from_buffer(CLIENT_BRUSH_CAPABILITY_SET.as_ref()).unwrap()
                ),
                CapabilitySet::OffscreenBitmapCache(
                    OffscreenBitmapCache::from_buffer(
                        CLIENT_OFFSCREEN_BITMAP_CAPABILITY_SET.as_ref()
                    )
                    .unwrap()
                ),
                CapabilitySet::VirtualChannel(
                    VirtualChannel::from_buffer(CLIENT_VIRTUAL_CHANNEL_CAPABILITY_SET.as_ref())
                        .unwrap()
                ),
                CapabilitySet::DrawNineGridCache(
                    CLIENT_DRAW_NINE_GRID_CACHE_CAPABILITY_SET.to_vec()
                ),
                CapabilitySet::DrawGdiPlus(CLIENT_DRAW_GDI_PLUS_CAPABILITY_SET.to_vec()),
            ],
        }
    };
}

#[test]
fn from_buffer_correctly_parses_server_demand_active() {
    let buffer = SERVER_DEMAND_ACTIVE_BUFFER.as_ref();

    assert_eq!(
        *SERVER_DEMAND_ACTIVE,
        ServerDemandActive::from_buffer(buffer).unwrap()
    );
}

#[test]
fn from_buffer_correctly_parses_client_demand_active_with_incomplete_capability_set() {
    let buffer = CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET_BUFFER.as_ref();

    assert_eq!(
        *CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET,
        ClientConfirmActive::from_buffer(buffer).unwrap()
    );
}

#[test]
fn from_buffer_correctly_parses_client_demand_active() {
    let buffer = CLIENT_DEMAND_ACTIVE_BUFFER.as_ref();

    assert_eq!(
        *CLIENT_DEMAND_ACTIVE,
        ClientConfirmActive::from_buffer(buffer).unwrap()
    );
}

#[test]
fn to_buffer_correctly_serializes_server_demand_active() {
    let data = SERVER_DEMAND_ACTIVE.clone();
    let expected_buffer = SERVER_DEMAND_ACTIVE_BUFFER.to_vec();

    let mut buff = Vec::new();
    data.to_buffer(&mut buff).unwrap();

    assert_eq!(expected_buffer, buff);
}

#[test]
fn to_buffer_correctly_serializes_client_demand_active_with_incomplete_capability_set() {
    let data = CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET.clone();
    let expected_buffer = CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET_BUFFER.to_vec();

    let mut buff = Vec::new();
    data.to_buffer(&mut buff).unwrap();

    assert_eq!(expected_buffer, buff);
}

#[test]
fn to_buffer_correctly_serializes_client_demand_active() {
    let data = CLIENT_DEMAND_ACTIVE.clone();
    let expected_buffer = CLIENT_DEMAND_ACTIVE_BUFFER.to_vec();

    let mut buff = Vec::new();
    data.to_buffer(&mut buff).unwrap();

    assert_eq!(expected_buffer, buff);
}

#[test]
fn buffer_length_is_correct_for_server_demand_active() {
    let data = SERVER_DEMAND_ACTIVE.clone();
    let expected_buffer_len = SERVER_DEMAND_ACTIVE_BUFFER.len();

    let len = data.buffer_length();

    assert_eq!(expected_buffer_len, len);
}

#[test]
fn buffer_length_is_correct_for_client_demand_active_with_incomplete_capability_set() {
    let data = CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET.clone();
    let expected_buffer_len = CLIENT_DEMAND_ACTIVE_WITH_INCOMPLETE_CAPABILITY_SET_BUFFER.len();

    let len = data.buffer_length();

    assert_eq!(expected_buffer_len, len);
}

#[test]
fn buffer_length_is_correct_for_client_demand_active() {
    let data = CLIENT_DEMAND_ACTIVE.clone();
    let expected_buffer_len = CLIENT_DEMAND_ACTIVE_BUFFER.len();

    let len = data.buffer_length();

    assert_eq!(expected_buffer_len, len);
}
