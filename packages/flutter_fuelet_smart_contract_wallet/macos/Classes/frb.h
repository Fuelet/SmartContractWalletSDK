#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_SmartContractWallet {
  struct wire_uint_8_list *bech32_address;
  struct wire_uint_8_list *r1_public_key;
  struct wire_uint_8_list *contract_id;
  struct wire_uint_8_list *recovery_private_key;
  struct wire_uint_8_list *node_url;
} wire_SmartContractWallet;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_connect__static_method__SmartContractWallet(int64_t port_,
                                                      struct wire_uint_8_list *r1_public_key,
                                                      struct wire_uint_8_list *recovery_private_key,
                                                      struct wire_uint_8_list *node_url);

void wire_deploy_contract__method__SmartContractWallet(int64_t port_,
                                                       struct wire_SmartContractWallet *that);

void wire_gen_transfer_tx_request__method__SmartContractWallet(int64_t port_,
                                                               struct wire_SmartContractWallet *that,
                                                               struct wire_uint_8_list *to_b256,
                                                               uint64_t amount,
                                                               struct wire_uint_8_list *asset);

void wire_send_tx__method__SmartContractWallet(int64_t port_,
                                               struct wire_SmartContractWallet *that,
                                               struct wire_uint_8_list *encoded_tx,
                                               struct wire_uint_8_list *signature);

struct wire_SmartContractWallet *new_box_autoadd_smart_contract_wallet_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_connect__static_method__SmartContractWallet);
    dummy_var ^= ((int64_t) (void*) wire_deploy_contract__method__SmartContractWallet);
    dummy_var ^= ((int64_t) (void*) wire_gen_transfer_tx_request__method__SmartContractWallet);
    dummy_var ^= ((int64_t) (void*) wire_send_tx__method__SmartContractWallet);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_smart_contract_wallet_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
