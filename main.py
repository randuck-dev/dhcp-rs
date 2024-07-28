import dhcppython

client = dhcppython.client.DHCPClient(send_to_port=50010, send_from_port=50011)

lease = client.get_lease(server="127.0.0.1")
print(lease)