<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PredicitiveSearch_LocalidadN4</name>
   <tag></tag>
   <elementGuidId>284ea25e-0fbd-4b3a-b8f9-6d70509c55d9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.cartoonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:queryN4>
         &lt;!--Optional:-->
         &lt;nivel1>AR&lt;/nivel1>
         &lt;!--Optional:-->
         &lt;nivel2>Buenos Aires&lt;/nivel2>
         &lt;!--Optional:-->
         &lt;nivel4>vicent&lt;/nivel4>
         &lt;!--Optional:-->
         &lt;address>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;status> &lt;/status>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/address>
      &lt;/soap:queryN4>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryCalle</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PredicitiveSearch_SOAP1</defaultValue>
      <description></description>
      <id>2866abb5-d641-4892-a76f-3c0b0b2ba2a5</id>
      <masked>false</masked>
      <name>PredictiveSearch_SOAP1</name>
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





assertThat(response.getResponseText()).contains('&lt;name>SAN VICENTE&lt;/name>&lt;relieved>true&lt;/relieved>&lt;synonymous>&lt;/synonymous>&lt;x>-58.420732&lt;/x>&lt;y>-35.026405&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;name>VICENTE CASARES&lt;/name>&lt;relieved>true&lt;/relieved>&lt;synonymous>&lt;/synonymous>&lt;x>-58.650336&lt;/x>&lt;y>-34.967420&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;name>VICENTE LOPEZ&lt;/name>&lt;relieved>true&lt;/relieved>&lt;synonymous>&lt;/synonymous>&lt;x>-58.474951&lt;/x>&lt;y>-34.525741&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;name>VICENTE PEREDA&lt;/name>&lt;relieved>false&lt;/relieved>&lt;synonymous>&lt;/synonymous>&lt;x>-59.780174&lt;/x>&lt;y>-36.696678&lt;/y>')</verificationScript>
   <wsdlAddress>${PredictiveSearch_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
