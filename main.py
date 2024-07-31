import dhcppython
import logging

logging.getLogger().setLevel(logging.DEBUG)
client = dhcppython.client.DHCPClient(
    send_to_port=50010, send_from_port=50011, max_retries=1
)

client_identifier = dhcppython.options.options.value_to_object(
    {"client_identifier": {"hwtype": 1, "hwaddr": "8C:45:00:1D:48:16"}}
)
lease = client.get_lease(
    server="127.0.0.1", options_list=dhcppython.options.OptionList([client_identifier])
)
print(lease)
