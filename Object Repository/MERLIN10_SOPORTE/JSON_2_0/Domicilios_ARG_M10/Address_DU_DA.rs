<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DA</name>
   <tag></tag>
   <elementGuidId>7de29628-1a68-4623-ac38-5228badcc426</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded; charset=utf-8</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${AddressJson}?level1=AR&amp;street=SAN%20JOSE%20DE%20FLORES%206001&amp;level4=Villa%20Ballester&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;postalCode=1653</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AddressJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>AddressJson</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'status', &quot;DU&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;DA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;BUENOS AIRES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;VILLA BALLESTER&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;SAN JOSE DE FLORES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;6001&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;00.000000&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;00.000000&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1653&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;6&quot;)

assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;&quot;,&quot;level4&quot;:&quot;VILLA GODOY CRUZ&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE DE FLORES&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;&quot;,&quot;fromStreetNumber&quot;:&quot;6201&quot;,&quot;toStreetNumber&quot;:&quot;6900&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;&quot;,&quot;level4&quot;:&quot;VILLA GENERAL JOSE TOMAS GUIDO&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE DE FLORES&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;&quot;,&quot;fromStreetNumber&quot;:&quot;4701&quot;,&quot;toStreetNumber&quot;:&quot;6200&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;&quot;,&quot;level4&quot;:&quot;VILLA GRANADEROS DE SAN MARTIN&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;&quot;,&quot;fromStreetNumber&quot;:&quot;1301&quot;,&quot;toStreetNumber&quot;:&quot;1600&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;&quot;,&quot;level4&quot;:&quot;VILLA GENERAL EUGENIO NECOCHEA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;&quot;,&quot;fromStreetNumber&quot;:&quot;1201&quot;,&quot;toStreetNumber&quot;:&quot;1800&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;&quot;,&quot;level4&quot;:&quot;VILLA BALLESTER&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;&quot;,&quot;fromStreetNumber&quot;:&quot;2001&quot;,&quot;toStreetNumber&quot;:&quot;2300&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;GENERAL SAN MARTIN&quot;,&quot;level4&quot;:&quot;VILLA GENERAL ANTONIO JOSE DE SUCRE&quot;,&quot;level5&quot;:&quot;&quot;,&quot;street&quot;:&quot;SAN JOSE DE FLORES&quot;,&quot;houseNumber&quot;:&quot;6001&quot;,&quot;latitude&quot;:&quot;00.000000&quot;,&quot;longitude&quot;:&quot;00.000000&quot;,&quot;postalCode&quot;:&quot;1653&quot;,&quot;fromStreetNumber&quot;:&quot;5801&quot;,&quot;toStreetNumber&quot;:&quot;6199&quot;}')

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;puerta&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;},{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;cpa&quot;,&quot;value&quot;:&quot;&quot;},{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.557521&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-58.557521&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-34.542114&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.542114&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;N&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
